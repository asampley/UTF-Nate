use itertools::Itertools;

use log::error;

use serde_json::value::Value;

use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;
use serenity::model::interactions::application_command::{
	ApplicationCommandInteraction, ApplicationCommandOptionType,
};

use crate::audio::PlayStyle;
use crate::util::interaction::create_interaction;
use crate::util::*;

mod generic;

#[group("voice")]
#[description("Commands to move the bot to voice channels and play clips.")]
#[commands(summon, banish, clip, play, volume, stop, skip, list, pause, unpause)]
pub struct Voice;

#[command]
#[only_in(guilds)]
#[help_available]
#[description("Summon the bot to the voice channel the user is currently in")]
pub async fn summon(ctx: &Context, msg: &Message) -> CommandResult {
	msg.respond_str(ctx, generic::summon(ctx, msg.guild_id, msg.author.id).await)
		.await?;

	Ok(())
}

pub async fn summon_interaction(
	ctx: &Context,
	interaction: &ApplicationCommandInteraction,
) -> serenity::Result<()> {
	interaction
		.respond_str(
			&ctx,
			generic::summon(ctx, interaction.guild_id, interaction.user.id).await,
		)
		.await
}

pub fn summon_interaction_create(
	command: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
	command
		.name("summon")
		.description("Summon the bot to your current voice channel")
}

#[command]
#[only_in(guilds)]
#[help_available]
#[description("Remove the bot from the voice channel it is in")]
pub async fn banish(ctx: &Context, msg: &Message) -> CommandResult {
	msg.respond_str(ctx, generic::banish(ctx, msg.guild_id).await)
		.await?;

	Ok(())
}

pub async fn banish_interaction(
	ctx: &Context,
	interaction: &ApplicationCommandInteraction,
) -> serenity::Result<()> {
	interaction
		.respond_str(&ctx, generic::banish(ctx, interaction.guild_id).await)
		.await
}

pub fn banish_interaction_create(
	command: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
	command
		.name("banish")
		.description("Banish the bot from its current voice channel")
}

#[command]
#[only_in(guilds)]
#[help_available]
#[description("Play the specified clip immediately")]
#[num_args(1)]
#[usage("<clip>")]
#[example("dota/bothello")]
#[example("bothello")]
pub async fn clip(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	let path = args.current();

	msg.respond_str(
		ctx,
		generic::play(ctx, PlayStyle::Clip, path, msg.guild_id).await,
	)
	.await?;

	Ok(())
}

pub async fn clip_interaction(
	ctx: &Context,
	interaction: &ApplicationCommandInteraction,
) -> serenity::Result<()> {
	let clip = interaction.data.options.iter().find_map(|option| {
		if option.name == "clip" {
			option.value.as_ref()
		} else {
			None
		}
	});

	let clip = match clip {
		Some(Value::String(clip)) => Some(clip.as_str()),
		None => None,
		Some(_) => {
			error!("Error in clip interaction expecting string argument");
			return interaction.respond_str(&ctx, "Internal bot error").await;
		}
	};

	interaction
		.respond_str(
			ctx,
			generic::play(ctx, PlayStyle::Clip, clip, interaction.guild_id).await,
		)
		.await
}

pub fn clip_interaction_create(
	command: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
	command
		.name("clip")
		.description("Play the specified clip immediately")
		.create_option(|option| {
			option
				.name("clip")
				.description("Clip to play")
				.kind(ApplicationCommandOptionType::String)
				.required(true)
		})
}

#[command]
#[only_in(guilds)]
#[help_available]
#[description(
	"Add a youtube video, playlist, search, or spotify song, playlist, or album to the queue"
)]
#[min_args(1)]
#[usage("<source>")]
#[example("arbitrary youtube search")]
#[example("https://www.youtube.com/watch?v=k2mFvwDTTt0")]
#[example("https://www.youtube.com/playlist?list=PLucOLpdAYaKW1IYuo84R4qIskTfj-ECDp")]
#[example("https://open.spotify.com/track/009bpReJuXgCv8G2MkJ5Y1")]
#[example("https://open.spotify.com/album/0G2RxSCixG5Nl6jpjwiw2g")]
#[example("https://open.spotify.com/playlist/2O18dCV9uoGTyxN5HLJkTo")]
pub async fn play(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	let query = args.raw().join(" ");
	let query = if query.len() == 0 {
		None
	} else {
		Some(query.as_str())
	};

	msg.respond_str(
		ctx,
		generic::play(ctx, PlayStyle::Play, query, msg.guild_id).await,
	)
	.await?;

	Ok(())
}

pub async fn play_interaction(
	ctx: &Context,
	interaction: &ApplicationCommandInteraction,
) -> serenity::Result<()> {
	let clip = interaction.data.options.iter().find_map(|option| {
		if option.name == "input" {
			option.value.as_ref()
		} else {
			None
		}
	});

	let clip = match clip {
		Some(Value::String(clip)) => Some(clip.as_str()),
		None => None,
		Some(_) => {
			error!("Error in play interaction expecting string argument");
			return interaction.respond_str(&ctx, "Internal bot error").await;
		}
	};

	interaction
		.respond_str(
			ctx,
			generic::play(ctx, PlayStyle::Play, clip, interaction.guild_id).await,
		)
		.await
}

pub fn play_interaction_create(
	cmd: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
	create_interaction(&PLAY_COMMAND, cmd).create_option(|option| {
		option
			.name("input")
			.description("Youtube or Spotify URL, or youtube search")
			.kind(ApplicationCommandOptionType::String)
			.required(true)
	})
}

#[command]
#[only_in(guilds)]
#[help_available]
#[description("Change volume of bot")]
#[min_args(0)]
#[max_args(2)]
#[usage("<play|clip> <volume>")]
#[example("")]
#[example("play")]
#[example("clip")]
#[example("play .25")]
#[example("clip 0.5")]
pub async fn volume(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
	let style = match args.remaining() {
		0 => None,
		_ => {
			Some(args
				.single::<PlayStyle>()
				.or_err_say(
					ctx,
					msg,
					"Expected either \"play\" or \"clip\" volume to be selected",
				)
				.await?)
		}
	};

	let volume = match args.remaining() {
		0 => None,
		_ => {
			Some(args
				.single::<f32>()
				.or_err_say(ctx, msg, "Volume must be a valid float between 0.0 and 1.0")
				.await?)
		}
	};

	msg.respond_str(
		ctx,
		generic::volume(ctx, style, msg.guild_id, volume).await,
	)
	.await?;

	Ok(())
}

pub async fn volume_interaction(
	ctx: &Context,
	interaction: &ApplicationCommandInteraction,
) -> serenity::Result<()> {
	let style = interaction.data.options.iter().find_map(|option| {
		if option.name == "style" {
			option.value.as_ref()
		} else {
			None
		}
	});

	let style = match style {
		Some(Value::String(style)) => style.parse::<PlayStyle>().ok(),
		None => None,
		Some(_) => {
			error!("Error in volume interaction expecting string argument");
			return interaction.respond_str(&ctx, "Internal bot error").await;
		}
	};

	let volume = interaction.data.options.iter().find_map(|option| {
		if option.name == "volume" {
			option.value.as_ref()
		} else {
			None
		}
	});

	let volume = match volume {
		Some(Value::Number(volume)) => volume.as_f64().map(|v| v as f32),
		None => None,
		Some(_) => {
			error!("Error in volume interaction expecting float argument");
			return interaction.respond_str(&ctx, "Internal bot error").await;
		}
	};

	interaction
		.respond_str(
			ctx,
			generic::volume(ctx, style, interaction.guild_id, volume).await,
		)
		.await
}

pub fn volume_interaction_create(
	cmd: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
	create_interaction(&VOLUME_COMMAND, cmd)
		.create_option(|option| {
			option
				.name("style")
				.description("Volume to set, either for play or clip commands")
				.kind(ApplicationCommandOptionType::String)
				.add_string_choice("play", "play")
				.add_string_choice("clip", "clip")
		})
		.create_option(|option| {
			option
				.name("volume")
				.description("Volume between 0.0 and 1.0")
				.kind(ApplicationCommandOptionType::Number)
		})
}

#[command]
#[only_in(guilds)]
#[help_available]
#[description("Stop all clips currently being played by the bot")]
pub async fn stop(ctx: &Context, msg: &Message) -> CommandResult {
	msg.respond_str(ctx, generic::stop(ctx, msg.guild_id).await)
		.await?;

	Ok(())
}

pub async fn stop_interaction(
	ctx: &Context,
	interaction: &ApplicationCommandInteraction,
) -> serenity::Result<()> {
	interaction
		.respond_str(ctx, generic::stop(ctx, interaction.guild_id).await)
		.await
}

pub fn stop_interaction_create(
	cmd: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
	create_interaction(&STOP_COMMAND, cmd)
}

#[command]
#[only_in(guilds)]
#[help_available]
#[description("Skip the current song in the queue")]
pub async fn skip(ctx: &Context, msg: &Message) -> CommandResult {
	msg.respond_str(ctx, generic::skip(ctx, msg.guild_id).await)
		.await?;

	Ok(())
}

pub async fn skip_interaction(
	ctx: &Context,
	interaction: &ApplicationCommandInteraction,
) -> serenity::Result<()> {
	interaction
		.respond_str(ctx, generic::skip(ctx, interaction.guild_id).await)
		.await
}

pub fn skip_interaction_create(
	cmd: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
	create_interaction(&SKIP_COMMAND, cmd)
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
		msg.respond_str(ctx, "Expected at most one path to be specified")
			.await?;
		return Ok(());
	}

	let path = args.current();
	msg.respond_str(ctx, generic::list(path).await).await?;

	return Ok(());
}

pub async fn list_interaction(
	ctx: &Context,
	interaction: &ApplicationCommandInteraction,
) -> serenity::Result<()> {
	let path = interaction.data.options.iter().find_map(|option| {
		if option.name == "path" {
			option.value.as_ref()
		} else {
			None
		}
	});

	let path = match path {
		Some(Value::String(path)) => Some(path.as_str()),
		None => None,
		Some(_) => {
			error!("Error in list interaction expecting string argument");
			return interaction.respond_str(&ctx, "Internal bot error").await;
		}
	};

	interaction
		.respond_str(ctx, generic::list(path).await)
		.await
}

pub fn list_interaction_create(
	cmd: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
	create_interaction(&LIST_COMMAND, cmd).create_option(|option| {
		option
			.name("path")
			.description("Path to list clips underneath")
			.kind(ApplicationCommandOptionType::String)
	})
}

#[command]
#[only_in(guilds)]
#[help_available]
#[description("Pause the queue")]
pub async fn pause(ctx: &Context, msg: &Message) -> CommandResult {
	msg.respond_str(ctx, generic::pause(ctx, msg.guild_id).await)
		.await?;

	Ok(())
}

pub async fn pause_interaction(
	ctx: &Context,
	interaction: &ApplicationCommandInteraction,
) -> serenity::Result<()> {
	interaction
		.respond_str(ctx, generic::pause(ctx, interaction.guild_id).await)
		.await
}

pub fn pause_interaction_create(
	cmd: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
	create_interaction(&PAUSE_COMMAND, cmd)
}

#[command]
#[only_in(guilds)]
#[help_available]
#[description("Unpause the queue")]
pub async fn unpause(ctx: &Context, msg: &Message) -> CommandResult {
	msg.respond_str(ctx, generic::unpause(ctx, msg.guild_id).await)
		.await?;

	Ok(())
}

pub async fn unpause_interaction(
	ctx: &Context,
	interaction: &ApplicationCommandInteraction,
) -> serenity::Result<()> {
	interaction
		.respond_str(ctx, generic::unpause(ctx, interaction.guild_id).await)
		.await
}

pub fn unpause_interaction_create(
	cmd: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
	create_interaction(&UNPAUSE_COMMAND, cmd)
}
