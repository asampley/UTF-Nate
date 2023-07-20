use askama::Template;

use axum::response::Html;

use axum_extra::extract::CookieJar;

use hyper::StatusCode;
use once_cell::sync::Lazy;

use ring::aead::LessSafeKey;

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
	success: bool,
	response: &'a str,
}

static FORMS: Lazy<HashMap<fn() -> Command, String>> = Lazy::new(|| {
	super::COMMAND_CREATES
		.iter()
		.map(|c| (c.create, render_form(&(c.create)(), c.help)))
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
		.map_err(|_| "Invalid token, please regenerate using the `/token` slash command")?
		.decrypt::<Token>(key)
		.map_err(|_| "Invalid token, please regenerate using the `/token` slash command")?;

	(&token)
		.try_into()
		.map_err(|_| "Token expired, please regenerate using the `/token` slash command".into())
}

pub fn response_string(response: Result<Response, Response>) -> String {
	response.unwrap_or_else(|e| e).text
}

pub fn render_response(response: Result<Response, Response>) -> Html<String> {
	ResponseTemplate {
		success: response.is_ok(),
		response: &markdown::to_html(&response_string(response)),
	}
	.render()
	.unwrap()
	.into()
}

fn render_form(command: &Command, help_md: &str) -> String {
	CommandFormTemplate {
		command,
		help: &markdown::to_html(help_md),
	}
	.render()
	.unwrap()
}

pub fn form_endpoint(command: fn() -> Command) -> Result<Html<&'static str>, StatusCode> {
	get_form(command).map(Html).ok_or(StatusCode::NOT_FOUND)
}
