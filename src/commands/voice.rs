use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::channel::Message;

use crate::audio::PlayStyle;
use crate::commands::{create_interaction, run};
use crate::util::*;

mod generic;

#[group("voice")]
#[description("Commands for info and settings for playback.")]
#[commands(volume, list)]
pub struct Voice;

#[command]
#[only_in(guilds)]
#[help_available]
#[description("Get or change the volume of the bot")]
#[min_args(0)]
#[max_args(2)]
#[usage("<get|play|clip> <volume?>")]
#[example("get")]
#[example("play")]
#[example("clip")]
#[example("play .25")]
#[example("clip 0.5")]
pub async fn volume(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
	let style = match args.remaining() {
		0 => None,
		_ => match args.parse::<PlayStyle>().ok() {
			Some(v) => Some(v),
			None => {
				if args.current().unwrap() == "get" {
					None
				} else {
					msg.respond_err(ctx, &"Specify \"get\", \"play\", or \"clip\"".into())
						.await?;

					return Ok(());
				}
			}
		},
	};

	args.advance();

	let volume = match args.remaining() {
		0 => None,
		_ => match args.single::<f32>() {
			Ok(volume) => Some(volume),
			Err(_) => {
				msg.respond_err(
					ctx,
					&"Volume must be a valid float between 0.0 and 1.0".into(),
				)
				.await?;

				return Ok(());
			}
		},
	};

	run(ctx, msg, generic::volume(ctx, style, msg.guild_id, volume)).await
}

pub async fn volume_interaction(
	ctx: &Context,
	int: &ApplicationCommandInteraction,
) -> serenity::Result<()> {
	let options = &int.data.options;

	let opt = get_option(options, "play")
		.or_else(|| get_option(options, "clip"))
		.map(|sub| {
			(
				sub.name.parse().ok(),
				get_option_f32(ctx, int, &sub.options, "volume"),
			)
		});

	let (style, volume) = if let Some((style, volume_fut_res)) = opt {
		(style, volume_fut_res.await?)
	} else {
		(None, None)
	};

	run(ctx, int, generic::volume(ctx, style, int.guild_id, volume)).await
}

pub fn volume_interaction_create(
	cmd: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
	create_interaction(&VOLUME_COMMAND, cmd)
		.create_option(|option| {
			option
				.name("get")
				.description("Get the volume for both play and clip commands")
				.kind(CommandOptionType::SubCommand)
		})
		.create_option(|option| {
			option
				.name("play")
				.description("Get or set the volume for play commands")
				.kind(CommandOptionType::SubCommand)
				.create_sub_option(|option| {
					option
						.name("volume")
						.description("Volume between 0.0 and 1.0")
						.kind(CommandOptionType::Number)
				})
		})
		.create_option(|option| {
			option
				.name("clip")
				.description("Get or set the volume for play commands")
				.kind(CommandOptionType::SubCommand)
				.create_sub_option(|option| {
					option
						.name("volume")
						.description("Volume between 0.0 and 1.0")
						.kind(CommandOptionType::Number)
				})
		})
}

#[command]
#[help_available]
#[description("List all the sections and/or clips available in the section")]
#[min_args(0)]
#[max_args(1)]
#[usage("<section?>")]
#[example("bnw")]
pub async fn list(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	if args.len() > 1 {
		msg.respond_err(ctx, &"Expected at most one path to be specified".into())
			.await?;
		return Ok(());
	}

	let path = args.current();

	run(ctx, msg, generic::list(path)).await
}

pub async fn list_interaction(
	ctx: &Context,
	int: &ApplicationCommandInteraction,
) -> serenity::Result<()> {
	let path = get_option_string(ctx, int, &int.data.options, "path").await?;

	run(ctx, int, generic::list(path)).await
}

pub fn list_interaction_create(
	cmd: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
	create_interaction(&LIST_COMMAND, cmd).create_option(|option| {
		option
			.name("path")
			.description("Path to list clips underneath")
			.kind(CommandOptionType::String)
	})
}
