use askama::Template;

use axum::response::Html;

use axum_extra::extract::CookieJar;

use futures::Future;

use once_cell::sync::Lazy;

use ring::aead::LessSafeKey;
use serde::Serialize;

use std::collections::HashMap;

use crate::encrypt::Encrypted;
use crate::http::Token;
use crate::util::{Command, Response};

#[derive(Template)]
#[template(path = "command_form.html")]
struct CommandFormTemplate<'a> {
	command: &'a Command,
	help: &'a str,
}

#[derive(Template)]
#[template(path = "response.html")]
struct ResponseTemplate<'a> {
	response: &'a str,
}

static FORMS: Lazy<HashMap<fn() -> Command, String>> = Lazy::new(|| {
	super::COMMAND_CREATES
		.iter()
		.map(|c| (c.create, form(&(c.create)(), c.help)))
		.collect()
});

pub enum TokenError {
	Expired,
}

impl TryFrom<&Token> for super::Source {
	type Error = TokenError;

	fn try_from(token: &Token) -> Result<Self, Self::Error> {
		if token.is_expired() {
			Err(TokenError::Expired)
		} else {
			Ok(Self {
				guild_id: token.guild_id,
				user_id: token.user_id,
			})
		}
	}
}

/// Get a form for a command via function item.
pub fn get_form(t: fn() -> Command) -> Option<&'static str> {
	FORMS.get(&t).map(|s| &**s)
}

pub fn extract_source(jar: &CookieJar, key: &LessSafeKey) -> Result<super::Source, Response> {
	let token: Token = TryInto::<Encrypted>::try_into(jar)
		.map_err(|_| "Invalid token, please regenerate")?
		.decrypt::<Token>(key)
		.map_err(|_| "Invalid token, please regenerate")?;

	(&token)
		.try_into()
		.map_err(|_| "Token expired, please regenerate".into())
}

pub fn response_to_string(response: Result<Response, Response>) -> String {
	match response {
		Ok(o) => format!("\u{2705} {}", o),
		Err(e) => format!("\u{274c} {}", e),
	}
}

pub fn response_to_html_string(response: Result<Response, Response>) -> String {
	ResponseTemplate {
		response: &markdown::to_html(&response_to_string(response)),
	}
	.render()
	.unwrap()
}

pub async fn run<F, I, O>(command: F, input: I) -> Html<String>
where
	F: Fn(I) -> O,
	O: Future<Output = Result<Response, Response>>,
	I: Serialize,
{
	Html(response_to_html_string(command(input).await))
}

pub fn form(command: &Command, help_md: &str) -> String {
	CommandFormTemplate {
		command,
		help: &markdown::to_html(help_md),
	}
	.render()
	.unwrap()
}

pub fn form_endpoint(command: fn() -> Command) -> Html<String> {
	Html(get_form(command).unwrap().to_string())
}
