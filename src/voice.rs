use once_cell::sync::Lazy;

use serde_json::value::Value;

use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;
use serenity::model::interactions::application_command::{
	ApplicationCommandInteraction,
	ApplicationCommandOptionType,
};
use regex::Regex;

use songbird::input::Input;

use std::fmt;
use std::path::{Path, PathBuf};

use crate::util::*;

mod generic;

static YOUTUBE: Lazy<Regex> = Lazy::new(|| {
	Regex::new("^https?://(www\\.youtube\\.com|youtu.be)/").unwrap()
});

#[group("voice")]
#[description("Commands to move the bot to voice channels and play clips.")]
#[commands(summon, banish, clip, play, volume, stop, skip, list)]
pub struct Voice;

pub fn clip_path() -> PathBuf {
	return Path::new("./resources/clips").canonicalize().unwrap();
}

#[derive(Debug)]
pub enum AudioError {
	Songbird(songbird::input::error::Error),
	Serenity(serenity::Error),
}

impl From<songbird::input::error::Error> for AudioError {
	fn from(e: songbird::input::error::Error) -> Self {
		AudioError::Songbird(e)
	}
}

impl From<serenity::Error> for AudioError {
	fn from(e: serenity::Error) -> Self {
		AudioError::Serenity(e)
	}
}

impl fmt::Display for AudioError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Debug::fmt(self, f)
	}
}

impl std::error::Error for AudioError {}

pub async fn audio_source(loc: &str) -> Result<Input, AudioError> {
	Ok(if YOUTUBE.is_match(loc) {
		songbird::ytdl(&loc).await?
	} else {
		match get_clip(&loc) {
			Some(clip) => songbird::ffmpeg(&clip).await?,
			None => Err(serenity::Error::Other("Could not find source"))?,
		}
	})
}

pub fn get_clip(loc: &str) -> Option<PathBuf> {
	let clip_path = clip_path();
	let mut play_path = clip_path.join(&loc);

	for ext in &["mp3", "wav"] {
		play_path.set_extension(ext);

		if valid_clip(&play_path) {
			return Some(play_path);
		}
	}

	None
}

fn valid_clip(path: &Path) -> bool {
	sandboxed_exists(&clip_path(), &path)
}

#[command]
#[only_in(guilds)]
#[help_available]
#[description("Summon the bot to the voice channel the user is currently in")]
pub async fn summon(ctx: &Context, msg: &Message) -> CommandResult {
	msg.channel_id.say(&ctx.http, generic::summon(ctx, msg.guild_id, msg.author.id).await).await?;

	Ok(())
}

pub async fn summon_interaction(
	ctx: &Context,
	interaction: &ApplicationCommandInteraction,
) -> serenity::Result<()> {
	interaction.respond_str(&ctx, generic::summon(ctx, interaction.guild_id, interaction.user.id).await).await
}

pub fn summon_interaction_create(
	command: &mut CreateApplicationCommand
) -> &mut CreateApplicationCommand {
	command.name("summon")
		.description("Summon the bot to your current voice channel")
}

#[command]
#[only_in(guilds)]
#[help_available]
#[description("Remove the bot from the voice channel it is in")]
pub async fn banish(ctx: &Context, msg: &Message) -> CommandResult {
	msg.channel_id.say(&ctx.http, generic::banish(ctx, msg.guild_id).await).await?;

	Ok(())
}

pub async fn banish_interaction(
	ctx: &Context,
	interaction: &ApplicationCommandInteraction,
) -> serenity::Result<()> {
	interaction.respond_str(&ctx, generic::banish(ctx, interaction.guild_id).await).await
}

pub fn banish_interaction_create(
	command: &mut CreateApplicationCommand
) -> &mut CreateApplicationCommand {
	command.name("banish")
		.description("Banish the bot from its current voice channel")
}

#[command]
#[only_in(guilds)]
#[help_available]
#[description("Play the specified clip immediately")]
#[num_args(1)]
#[usage("<clip>")]
#[example("bnw/needoffspring")]
pub async fn clip(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	let path = args.current();

	msg.channel_id.say(
		&ctx.http,
		generic::play(ctx, generic::PlayType::PlayNow, path, msg.guild_id).await,
	).await?;

	Ok(())
}

pub async fn clip_interaction(
	ctx: &Context,
	interaction: &ApplicationCommandInteraction,
) -> serenity::Result<()> {
	let clip = interaction.data.options.iter()
		.find_map(|option| if option.name == "clip" { option.value.as_ref() } else { None });

	let clip = match clip {
		Some(Value::String(clip)) => Some(clip.as_str()),
		None => None,
		Some(_) => {
			eprintln!("Error in clip interaction expecting string argument");
			return interaction.respond_str(&ctx, "Internal bot error").await;
		}
	};

	interaction.respond_str(
		ctx,
		generic::play(ctx, generic::PlayType::PlayNow, clip, interaction.guild_id).await,
	).await
}

pub fn clip_interaction_create(
	command: &mut CreateApplicationCommand
) -> &mut CreateApplicationCommand {
	command.name("clip")
		.description("Play the specified clip immediately")
		.create_option(|option|
			option
				.name("clip")
				.description("Clip to play")
				.kind(ApplicationCommandOptionType::String)
				.required(true)
		)
}

#[command]
#[aliases(q)]
#[only_in(guilds)]
#[help_available]
#[description("Add the specified clip to the play queue")]
#[num_args(1)]
#[usage("<clip>")]
#[example("bnw/needoffspring")]
pub async fn play(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	let path = args.current();

	msg.channel_id.say(
		&ctx.http,
		generic::play(ctx, generic::PlayType::Queue, path, msg.guild_id).await,
	).await?;

	Ok(())
}

pub async fn play_interaction(
	ctx: &Context,
	interaction: &ApplicationCommandInteraction,
) -> serenity::Result<()> {
	let clip = interaction.data.options.iter()
		.find_map(|option| if option.name == "clip" { option.value.as_ref() } else { None });

	let clip = match clip {
		Some(Value::String(clip)) => Some(clip.as_str()),
		None => None,
		Some(_) => {
			eprintln!("Error in play interaction expecting string argument");
			return interaction.respond_str(&ctx, "Internal bot error").await;
		}
	};

	interaction.respond_str(
		ctx,
		generic::play(ctx, generic::PlayType::Queue, clip, interaction.guild_id).await,
	).await
}

pub fn play_interaction_create(
	command: &mut CreateApplicationCommand
) -> &mut CreateApplicationCommand {
	command.name("play")
		.description("Add the specified clip to the play")
		.create_option(|option|
			option
				.name("clip")
				.description("Clip to play")
				.kind(ApplicationCommandOptionType::String)
				.required(true)
		)
}

#[command]
#[only_in(guilds)]
#[help_available]
#[description("Change volume of bot")]
#[num_args(1)]
#[usage("<volume>")]
#[example("0.5")]
pub async fn volume(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
	let volume = args.single::<f32>().or_err_say(ctx, msg, "Volume must be a valid float between 0.0 and 1.0").await?;

	msg.channel_id.say(&ctx.http, generic::volume(ctx, msg.guild_id, Some(volume)).await).await?;

	Ok(())
}

pub async fn volume_interaction(
	ctx: &Context,
	interaction: &ApplicationCommandInteraction,
) -> serenity::Result<()> {
	let volume = interaction.data.options.iter()
		.find_map(|option| if option.name == "volume" { option.value.as_ref() } else { None });

	let volume = match volume {
		Some(Value::Number(volume)) => volume.as_f64().map(|v| v as f32),
		None => None,
		Some(_) => {
			eprintln!("Error in volume interaction expecting float argument");
			return interaction.respond_str(&ctx, "Internal bot error").await;
		}
	};

	interaction.respond_str(
		ctx,
		generic::volume(ctx, interaction.guild_id, volume).await,
	).await
}

pub fn volume_interaction_create(
	command: &mut CreateApplicationCommand
) -> &mut CreateApplicationCommand {
	command.name("volume")
		.description("Change volume of bot")
		.create_option(|option|
			option
				.name("volume")
				.description("Volume between 0.0 and 1.0")
				.kind(ApplicationCommandOptionType::Number)
				.required(true)
		)
}

#[command]
#[only_in(guilds)]
#[help_available]
#[description("Stop all clips currently being played by the bot")]
pub async fn stop(ctx: &Context, msg: &Message) -> CommandResult {
	msg.channel_id.say(&ctx.http, generic::stop(ctx, msg.guild_id).await).await?;

	Ok(())
}

pub async fn stop_interaction(
	ctx: &Context,
	interaction: &ApplicationCommandInteraction,
) -> serenity::Result<()> {
	interaction.respond_str(
		ctx,
		generic::stop(ctx, interaction.guild_id).await,
	).await
}

pub fn stop_interaction_create(
	command: &mut CreateApplicationCommand
) -> &mut CreateApplicationCommand {
	command.name("stop")
		.description("Stop all clips currently being played by the bot")
}

#[command]
#[only_in(guilds)]
#[help_available]
#[description("Skip the current song in the queue")]
pub async fn skip(ctx: &Context, msg: &Message) -> CommandResult {
	msg.channel_id.say(&ctx.http, generic::skip(ctx, msg.guild_id).await).await?;

	Ok(())
}

pub async fn skip_interaction(
	ctx: &Context,
	interaction: &ApplicationCommandInteraction,
) -> serenity::Result<()> {
	interaction.respond_str(
		ctx,
		generic::skip(ctx, interaction.guild_id).await,
	).await
}

pub fn skip_interaction_create(
	command: &mut CreateApplicationCommand
) -> &mut CreateApplicationCommand {
	command.name("skip")
		.description("Skip the current song in the queue")
}

#[command]
#[help_available]
#[description("List all the sections and/or clips available in the section")]
#[min_args(0)]
#[max_args(1)]
#[usage("[section]")]
#[example("bnw")]
pub async fn list(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	if args.len() > 1 {
		msg.channel_id
			.say(&ctx.http, "Expected at most one path to be specified")
			.await?;
		return Ok(());
	}

	let path = args.current();
	msg.channel_id.say(&ctx.http, generic::list(path).await).await?;

	return Ok(());
}

pub async fn list_interaction(
	ctx: &Context,
	interaction: &ApplicationCommandInteraction,
) -> serenity::Result<()> {
	let path = interaction.data.options.iter()
		.find_map(|option| if option.name == "path" { option.value.as_ref() } else { None });

	let path = match path {
		Some(Value::String(path)) => Some(path.as_str()),
		None => None,
		Some(_) => {
			eprintln!("Error in list interaction expecting string argument");
			return interaction.respond_str(&ctx, "Internal bot error").await;
		}
	};

	interaction.respond_str(ctx, generic::list(path).await).await
}

pub fn list_interaction_create(
	command: &mut CreateApplicationCommand
) -> &mut CreateApplicationCommand {
	command.name("list")
		.description("List all the sections and/or clips available in the section")
		.create_option(|option|
			option
				.name("path")
				.description("Path to list clips underneath")
				.kind(ApplicationCommandOptionType::String)
		)
}
