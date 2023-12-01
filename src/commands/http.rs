use askama::Template;

use axum::body::HttpBody;
use axum::handler::Handler;
use axum::response::Html;
use axum::routing::get;
use axum::Router;

use axum_extra::extract::CookieJar;

use hyper::StatusCode;
use once_cell::sync::Lazy;

use ring::aead::LessSafeKey;

use std::collections::HashMap;

use crate::commands::CustomData;
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

pub trait FormRouter<S, B> {
	fn form_route<T>(self, create: fn() -> Command, http_call: impl Handler<T, S, B>) -> Self
	where
		T: 'static;
}

impl<S, B> FormRouter<S, B> for Router<S, B>
where
	S: Clone + Send + Sync + 'static,
	B: Send + HttpBody + 'static,
{
	fn form_route<T>(self, create: fn() -> Command, http_call: impl Handler<T, S, B>) -> Self
	where
		T: 'static,
	{
		let command = create();

		self.route(
			&String::from_iter(["/", &command.name]),
			get(move || async move { form_endpoint(create) }),
		)
		.route(
			&String::from_iter(["/", &command.name, "/run"]),
			get(http_call),
		)
	}
}

static FORMS: Lazy<HashMap<fn() -> Command, String>> = Lazy::new(|| {
	super::COMMAND_CREATES
		.iter()
		.filter_map(|create| {
			let command = create();

			command
				.custom_data
				.downcast_ref::<CustomData>()
				.map(|data| (*create, render_form(&command, (data.help_md)())))
		})
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
				channel_id: None,
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
