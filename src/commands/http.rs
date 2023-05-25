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

static FORMS: Lazy<HashMap<fn() -> Command, String>> = Lazy::new(|| {
	super::COMMAND_CREATES
		.iter()
		.map(|c| (*c, form(&c())))
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
	html::text_content::Paragraph::builder()
		.text(markdown::to_html(&response_to_string(response)))
		.build()
		.to_string()
}

pub async fn run<F, I, O>(
	command: F,
	poise: fn() -> Command,
	help_md: &str,
	input: Option<I>,
) -> Html<String>
where
	F: Fn(I) -> O,
	O: Future<Output = Result<Response, Response>>,
	I: Serialize,
{
	let response = match input {
		Some(i) => Some(command(i).await),
		None => None,
	};

	let mut html = String::new();

	if let Some(f) = get_form(poise) {
		html.push_str(f)
	};

	if let Some(r) = response {
		html.push_str(&response_to_html_string(r));
	}

	html.push_str(&markdown::to_html(help_md));

	Html(html)
}

pub fn form(c: &Command) -> String {
	let mut form = html::forms::Form::builder();

	for p in c.parameters.iter() {
		form.label(|e| e.for_(p.name.clone()).text(p.name.clone()))
			.input(|e| e.type_("text").name(p.name.clone()));
	}

	form.button(|e| e.type_("submit").text("Submit"));

	form.build().to_string()
}
