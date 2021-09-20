use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;
use serenity::model::interactions::application_command::{
	ApplicationCommandInteraction, ApplicationCommandOptionType,
};

use crate::commands::{create_interaction, run};
use crate::util::*;

mod generic;

#[group("queue")]
#[description("Commands for viewing and controlling the queue")]
#[commands(stop, skip, pause, unpause, queue, shuffle, shufflenow)]
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
#[description("Skip the current song in the queue")]
#[max_args(1)]
#[usage("<number?>")]
#[example("")]
#[example("3")]
pub async fn skip(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
	let skip = match args.remaining() {
		0 => None,
		_ => match args.single::<usize>() {
			Ok(skip) => Some(skip),
			Err(_) => {
				msg.respond_err(ctx, &"Skip count must be a positive integer".into())
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
	let skip = match get_option_usize(ctx, int, &int.data.options, "count").await {
		Ok(value) => value,
		Err(result) => return result,
	};

	run(ctx, int, generic::skip(ctx, int.guild_id, skip)).await
}

pub fn skip_interaction_create(
	cmd: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
	create_interaction(&SKIP_COMMAND, cmd).create_option(|option| {
		option
			.name("count")
			.description("Number of clips to skip")
			.kind(ApplicationCommandOptionType::Integer)
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
