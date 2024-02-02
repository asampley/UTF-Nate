//! All commands are defined under this module.
//!
//! Commands can be created for adding to the bot using [`commands()`].

pub mod external;
pub mod help;
pub mod herald;
#[cfg(feature = "http-interface")]
pub mod http;
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
use serenity::http::Http;
use serenity::model::id::{ChannelId, GuildId, UserId};
use serenity::prelude::{RwLock, TypeMap};

use std::sync::Arc;

use crate::util::{Command, CommandResult, Context, Respond, Response};

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

pub struct CustomData {
	pub help_md: fn() -> &'static str,
}

impl CustomData {
	pub fn new(help_md: fn() -> &'static str) -> Self {
		Self { help_md }
	}
}

#[derive(Debug)]
pub struct Source {
	guild_id: Option<GuildId>,
	channel_id: Option<ChannelId>,
	user_id: UserId,
}

impl From<&Context<'_>> for Source {
	fn from(ctx: &Context<'_>) -> Self {
		Source {
			guild_id: ctx.guild_id(),
			channel_id: Some(ctx.channel_id()),
			user_id: ctx.author().id,
		}
	}
}

#[derive(Clone)]
pub struct BotState {
	pub data: Arc<RwLock<TypeMap>>,
	pub cache: Arc<Cache>,
	pub http: Arc<Http>,
}

impl From<Context<'_>> for BotState {
	fn from(ctx: Context<'_>) -> Self {
		Self {
			data: ctx.serenity_context().data.clone(),
			cache: ctx.serenity_context().cache.clone(),
			http: ctx.serenity_context().http.clone(),
		}
	}
}

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
	let _typing_lock = match ctx {
		Context::Application(context) if context.command.ephemeral => {
			ctx.defer_ephemeral().await.map(|_| None)
		}
		_ => ctx.defer_or_broadcast().await,
	}?;

	ctx.respond(f.await.as_ref()).await?;

	Ok(())
}
