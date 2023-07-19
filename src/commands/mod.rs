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

pub struct CommandInfo {
	pub create: fn() -> Command,
	pub help: &'static str,
}

impl CommandInfo {
	const fn new(create: fn() -> Command, help: &'static str) -> Self {
		CommandInfo { create, help }
	}
}

pub static COMMAND_CREATES: &[CommandInfo] = &[
	CommandInfo::new(external::poise::cmd, external::cmd_help()),
	CommandInfo::new(external::poise::cmdlist, external::cmdlist_help()),
	CommandInfo::new(help::poise::help, help::help_help()),
	CommandInfo::new(herald::poise::intro, herald::intro_help()),
	CommandInfo::new(herald::poise::introbot, herald::introbot_help()),
	CommandInfo::new(herald::poise::outro, herald::outro_help()),
	CommandInfo::new(join::poise::summon, join::summon_help()),
	CommandInfo::new(join::poise::banish, join::banish_help()),
	CommandInfo::new(play::poise::clip, play::clip_help()),
	CommandInfo::new(play::poise::play, play::play_help()),
	CommandInfo::new(play::poise::playnext, play::playnext_help()),
	CommandInfo::new(play::poise::playnow, play::playnow_help()),
	CommandInfo::new(queue::poise::stop, queue::stop_help()),
	CommandInfo::new(queue::poise::skip, queue::skip_help()),
	CommandInfo::new(queue::poise::pause, queue::pause_help()),
	CommandInfo::new(queue::poise::unpause, queue::unpause_help()),
	CommandInfo::new(queue::poise::queue, queue::queue_help()),
	CommandInfo::new(queue::poise::shuffle, queue::shuffle_help()),
	CommandInfo::new(queue::poise::shufflenow, queue::shufflenow_help()),
	CommandInfo::new(queue::poise::r#loop, queue::loop_help()),
	CommandInfo::new(roll::poise::roll, roll::roll_help()),
	#[cfg(feature = "http-interface")]
	CommandInfo::new(token::poise::token, token::token_help()),
	CommandInfo::new(unicode::poise::unicode, unicode::unicode_help()),
	CommandInfo::new(voice::poise::volume, voice::volume_help()),
	CommandInfo::new(voice::poise::list, voice::list_help()),
];

/// Create a vector containing all the commands.
pub fn commands() -> Vec<Command> {
	COMMAND_CREATES.iter().map(|c| (c.create)()).collect()
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
