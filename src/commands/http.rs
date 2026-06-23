use askama::Template;

use axum::Router;
use axum::extract::State;
use axum::handler::Handler;
use axum::response::Html;
use axum::routing::get;

use axum_extra::extract::CookieJar;

use hyper::StatusCode;

use ring::aead::LessSafeKey;

use std::collections::HashMap;
use std::sync::LazyLock;

use crate::commands::{BotState, CustomData};
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

#[derive(Template)]
#[template(path = "command_list.html")]
struct CommandListTemplate<'a> {
	bot_name: &'a str,
	commands: &'a [&'a Command],
}

pub trait FormRouter<S> {
	fn form_route<T>(self, command: &Command, http_call: impl Handler<T, S>) -> Self
	where
		T: 'static;
}

impl<S> FormRouter<S> for Router<S>
where
	S: Clone + Send + Sync + 'static,
{
	fn form_route<T>(self, command: &Command, http_call: impl Handler<T, S>) -> Self
	where
		T: 'static,
	{
		let endpoint = form_endpoint(command);
		self.route(
			&String::from_iter(["/", &command.identifying_name]),
			get(move || async move { endpoint }),
		)
		.route(
			&String::from_iter(["/", &command.identifying_name, "/run"]),
			get(http_call),
		)
	}
}

static FORMS: LazyLock<HashMap<String, String>> = LazyLock::new(|| {
	let mut map = HashMap::new();

	fn add_recursive(map: &mut HashMap<String, String>, command: &Command) {
		if let Some(form) = command
			.custom_data
			.downcast_ref::<CustomData>()
			.map(|data| render_form(command, (data.help_md)()))
		{
			map.insert(command.identifying_name.clone(), form);
		}

		for subcommand in &command.subcommands {
			add_recursive(map, subcommand);
		}
	}

	for command in &*super::COMMANDS {
		add_recursive(&mut map, command);
	}

	map
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
pub fn get_form(command: &Command) -> Option<&'static str> {
	FORMS.get(&command.identifying_name).map(|s| &**s)
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
		response: &markdown::to_html(
			&response_string(response)
				// discord keeps newlines
				.replace("\n", "  \n"),
		),
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

pub fn form_endpoint(command: &Command) -> axum::response::Result<Html<&'static str>, StatusCode> {
	get_form(command).map(Html).ok_or(StatusCode::NOT_FOUND)
}

pub async fn command_list(
	State(state): State<BotState>,
) -> axum::response::Result<Html<String>, StatusCode> {
	let mut commands = Vec::new();

	super::for_each_recursive(|command| {
		if command.identifying_name != "token" {
			commands.push(command);
		}
	});

	let commands = &commands;

	CommandListTemplate {
		bot_name: state.cache.current_user().display_name(),
		commands,
	}
	.render()
	.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
	.map(Into::into)
}
