//! All commands are defined under this module.
//!
//! Commands can be created for adding to the bot using [`commands()`].

pub mod external;
pub mod help;
pub mod herald;
pub mod join;
pub mod play;
pub mod queue;
pub mod roll;
#[cfg(feature = "http-interface")]
pub mod token;
pub mod unicode;
pub mod voice;

use futures::Future;

use serenity::cache::Cache;
use serenity::model::prelude::{GuildId, UserId};
use serenity::prelude::{RwLock, TypeMap};

use std::sync::Arc;

use crate::util::{Command, CommandResult, Context, Respond, Response};

pub struct CustomData {
	help_md: fn() -> &'static str,
}

#[derive(Debug)]
pub struct Source {
	guild_id: Option<GuildId>,
	user_id: UserId,
}

impl From<&Context<'_>> for Source {
	fn from(ctx: &Context<'_>) -> Self {
		Source {
			guild_id: ctx.guild_id(),
			user_id: ctx.author().id,
		}
	}
}

#[derive(Clone)]
pub struct BotState {
	pub data: Arc<RwLock<TypeMap>>,
	pub cache: Arc<Cache>,
}

impl From<Context<'_>> for BotState {
	fn from(ctx: Context<'_>) -> Self {
		Self {
			data: ctx.discord().data.clone(),
			cache: ctx.discord().cache.clone(),
		}
	}
}

pub static COMMAND_CREATES: &[fn() -> Command] = &[
	external::poise::cmd,
	external::poise::cmdlist,
	help::poise::help,
	herald::poise::intro,
	herald::poise::introbot,
	herald::poise::outro,
	join::poise::summon,
	join::poise::banish,
	play::poise::clip,
	play::poise::play,
	play::poise::playnext,
	play::poise::playnow,
	queue::poise::stop,
	queue::poise::skip,
	queue::poise::pause,
	queue::poise::unpause,
	queue::poise::queue,
	queue::poise::shuffle,
	queue::poise::shufflenow,
	queue::poise::r#loop,
	roll::poise::roll,
	#[cfg(feature = "http-interface")]
	token::poise::token,
	unicode::poise::unicode,
	voice::poise::volume,
	voice::poise::list,
];

/// Create a vector containing all the commands.
pub fn commands() -> Vec<Command> {
	COMMAND_CREATES.iter().map(|c| c()).collect()
}

/// Await a command `f`, and then reply to the initiating message with the
/// response from the command.
pub async fn run<F>(ctx: &Context<'_>, f: F) -> CommandResult
where
	F: Future<Output = Result<Response, Response>>,
{
	ctx.respond(f.await.as_ref()).await?;

	Ok(())
}

#[cfg(feature = "http-interface")]
pub mod http {
	use axum::response::Html;

	use axum_extra::extract::CookieJar;

	use chrono::{DateTime, Utc};

	use futures::Future;

	use once_cell::sync::Lazy;

	use ring::aead::LessSafeKey;
	use serde::{Deserialize, Serialize};

	use serenity::model::prelude::{GuildId, UserId};

	use std::collections::HashMap;

	use crate::commands::token::Encrypted;
	use crate::util::{Command, Response};

	static FORMS: Lazy<HashMap<fn() -> Command, String>> = Lazy::new(|| {
		super::COMMAND_CREATES
			.iter()
			.map(|c| (*c, form(&c())))
			.collect()
	});

	/// Token that is used for the web interface.
	///
	/// Contains details of how the command was called.
	#[derive(Clone, Debug, Deserialize, Serialize)]
	pub struct Token {
		/// Guild id for the command, which is `None` when there is no guild.
		pub guild_id: Option<GuildId>,

		/// User id that invoked the command. Must always be set.
		pub user_id: UserId,

		/// Expiry timestamp for token
		pub expiry: DateTime<Utc>,
	}

	pub enum TokenError {
		Expired,
	}

	impl TryFrom<&Token> for super::Source {
		type Error = TokenError;

		fn try_from(token: &Token) -> Result<Self, Self::Error> {
			if token.expiry < Utc::now() {
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
}
