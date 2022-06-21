use tracing::{error, info};

use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;
use serenity::model::interactions::application_command::{
	ApplicationCommandInteraction, ApplicationCommandOptionType,
};

use crate::commands::{create_interaction, run};
use crate::parser::set;
use crate::util::*;

mod generic;

use generic::LoopArg;

#[group("queue")]
#[description("Commands for viewing and controlling the queue")]
#[commands(stop, skip, loop, pause, unpause, queue, shuffle, shufflenow)]
pub struct Queue;

#[command]
#[only_in(guilds)]
#[help_available]
#[description("Stop all clips currently being played by the bot")]
pub async fn stop(ctx: &Context, msg: &Message) -> CommandResult {
	run(ctx, msg, generic::stop(ctx, msg.guild_id)).await
}

pub async fn stop_interaction(
	ctx: &Context,
	int: &ApplicationCommandInteraction,
) -> serenity::Result<()> {
	run(ctx, int, generic::stop(ctx, int.guild_id)).await
}

pub fn stop_interaction_create(
	cmd: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
	create_interaction(&STOP_COMMAND, cmd)
}

#[command]
#[only_in(guilds)]
#[help_available]
#[description("Skip the current song in the queue, or select a list and/or range of songs to skip")]
#[max_args(1)]
#[usage("<selection?>")]
#[example("")]
#[example("2,3,4")]
#[example("4-7,9,0-2")]
pub async fn skip(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	let skip = match args.remaining() {
		0 => Vec::new(),
		_ => match set(args.current().unwrap()) {
			Ok(skip) => skip.1,
			Err(e) => {
				info!("Unable to parse skip selection: {:?}", e);

				msg.respond_err(
					ctx,
					&"Skip selection did not parse.
						Please enter a comma separated list of numbers and ranges without spaces.
						e.g. 4-7,9,0-2"
						.into(),
				)
				.await?;

				return Ok(());
			}
		},
	};

	run(ctx, msg, generic::skip(ctx, msg.guild_id, skip)).await
}

pub async fn skip_interaction(
	ctx: &Context,
	int: &ApplicationCommandInteraction,
) -> serenity::Result<()> {
	let skip = get_option_string(ctx, int, &int.data.options, "selection").await?;

	let skip = match skip.map(|s| set(s)) {
		None => Vec::new(),
		Some(Ok(skip)) => skip.1,
		Some(Err(e)) => {
			info!("Unable to parse skip selection: {:?}", e);

			int.respond_err(
				ctx,
				&"Skip selection did not parse.
					Please enter a comma separated list of numbers and ranges without spaces.
					e.g. 4-7,9,0-2"
					.into(),
			)
			.await?;

			return Ok(());
		}
	};

	run(ctx, int, generic::skip(ctx, int.guild_id, skip)).await
}

pub fn skip_interaction_create(
	cmd: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
	create_interaction(&SKIP_COMMAND, cmd).create_option(|option| {
		option
			.name("selection")
			.description("Range or index of songs to skip, separated by commas")
			.kind(ApplicationCommandOptionType::String)
	})
}

#[command]
#[only_in(guilds)]
#[help_available]
#[description("Pause the queue")]
pub async fn pause(ctx: &Context, msg: &Message) -> CommandResult {
	run(ctx, msg, generic::pause(ctx, msg.guild_id)).await
}

pub async fn pause_interaction(
	ctx: &Context,
	int: &ApplicationCommandInteraction,
) -> serenity::Result<()> {
	run(ctx, int, generic::pause(ctx, int.guild_id)).await
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
	run(ctx, msg, generic::unpause(ctx, msg.guild_id)).await
}

pub async fn unpause_interaction(
	ctx: &Context,
	int: &ApplicationCommandInteraction,
) -> serenity::Result<()> {
	run(ctx, int, generic::unpause(ctx, int.guild_id)).await
}

pub fn unpause_interaction_create(
	cmd: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
	create_interaction(&UNPAUSE_COMMAND, cmd)
}

#[command]
#[only_in(guilds)]
#[help_available]
#[description("Show the current queue of songs")]
pub async fn queue(ctx: &Context, msg: &Message) -> CommandResult {
	run(ctx, msg, generic::queue(ctx, msg.guild_id)).await
}

pub async fn queue_interaction(
	ctx: &Context,
	int: &ApplicationCommandInteraction,
) -> serenity::Result<()> {
	run(ctx, int, generic::queue(ctx, int.guild_id)).await
}

pub fn queue_interaction_create(
	cmd: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
	create_interaction(&QUEUE_COMMAND, cmd)
}

async fn shuffle_type_command(ctx: &Context, msg: &Message, starting_from: usize) -> CommandResult {
	run(ctx, msg, generic::shuffle(ctx, msg.guild_id, starting_from)).await
}

async fn shuffle_type_interaction(
	ctx: &Context,
	int: &ApplicationCommandInteraction,
	starting_from: usize,
) -> serenity::Result<()> {
	run(ctx, int, generic::shuffle(ctx, int.guild_id, starting_from)).await
}

#[command]
#[only_in(guilds)]
#[help_available]
#[description("Shuffle the queue of songs, after the current song")]
pub async fn shuffle(ctx: &Context, msg: &Message) -> CommandResult {
	shuffle_type_command(ctx, msg, 1).await
}

pub async fn shuffle_interaction(
	ctx: &Context,
	int: &ApplicationCommandInteraction,
) -> serenity::Result<()> {
	shuffle_type_interaction(ctx, int, 1).await
}

pub fn shuffle_interaction_create(
	cmd: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
	create_interaction(&SHUFFLE_COMMAND, cmd)
}

#[command]
#[only_in(guilds)]
#[help_available]
#[description("Shuffle the queue of songs, including the current song")]
pub async fn shufflenow(ctx: &Context, msg: &Message) -> CommandResult {
	shuffle_type_command(ctx, msg, 0).await
}

pub async fn shufflenow_interaction(
	ctx: &Context,
	int: &ApplicationCommandInteraction,
) -> serenity::Result<()> {
	shuffle_type_interaction(ctx, int, 0).await
}

pub fn shufflenow_interaction_create(
	cmd: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
	create_interaction(&SHUFFLENOW_COMMAND, cmd)
}

#[command]
#[only_in(guilds)]
#[help_available]
#[description("Set how many time the current song should loop")]
#[min_args(1)]
#[max_args(1)]
#[usage("<loop times>")]
#[example("on")]
#[example("off")]
#[example("3")]
pub async fn r#loop(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	let looop = match args.parse::<LoopArg>() {
		Ok(looop) => looop,
		Err(e) => {
			info!("Unable to parse loop argument: {}", e);

			msg.respond_err(
				ctx,
				&"Loop argument did not parse.
					Please enter the word \"on\" or \"off\" or enter a non-negative integer"
					.into(),
			)
			.await?;

			return Ok(());
		}
	};

	run(ctx, msg, generic::r#loop(ctx, msg.guild_id, looop)).await
}

pub async fn loop_interaction(
	ctx: &Context,
	int: &ApplicationCommandInteraction,
) -> serenity::Result<()> {
	let looop = match get_option_string(ctx, int, &int.data.options, "count").await? {
		Some(looop) => match looop.parse() {
			Ok(looop) => looop,
			Err(e) => {
				info!("Unable to parse loop argument: {}", e);

				int.respond_err(
					ctx,
					&"Loop argument did not parse.
						Please enter the word \"on\" or \"off\" or enter a non-negative integer"
						.into(),
				)
				.await?;

				return Ok(());
			}
		},
		None => {
			error!("No loop argument passed");
			int.respond_err(
				ctx,
				&"Please enter the word \"on\" or \"off\" or enter a non-negative integer".into(),
			)
			.await?;

			return Ok(());
		}
	};

	run(ctx, int, generic::r#loop(ctx, int.guild_id, looop)).await
}

pub fn loop_interaction_create(
	cmd: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
	create_interaction(&LOOP_COMMAND, cmd).create_option(|option| {
		option
			.name("count")
			.description("Number of loops, or \"on\" to loop forever, \"off\" to stop")
			.kind(ApplicationCommandOptionType::String)
			.required(true)
	})
}
