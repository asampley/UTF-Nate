use crate::commands::{run, CustomData};
use crate::parser::Selection;
use crate::util::*;

pub use super::LoopArg;
pub use super::ParseLoopArgError;
use super::{LoopArgs, SkipArgs};

#[poise::command(
	category = "queue",
	prefix_command,
	slash_command,
	guild_only,
	custom_data = "CustomData::new(super::stop_help)"
)]
pub async fn stop(ctx: Context<'_>) -> CommandResult {
	run(&ctx, super::stop(&ctx.into(), &(&ctx).into())).await
}

#[poise::command(
	category = "queue",
	prefix_command,
	slash_command,
	guild_only,
	custom_data = "CustomData::new(super::skip_help)"
)]
pub async fn skip(
	ctx: Context<'_>,
	#[description = "Range or index of songs to skip, separated by commas"] selection: Option<
		Selection<usize>,
	>,
) -> CommandResult {
	run(
		&ctx,
		super::skip(
			&ctx.into(),
			&(&ctx).into(),
			&SkipArgs {
				skip_set: selection,
			},
		),
	)
	.await
}

#[poise::command(
	category = "queue",
	prefix_command,
	slash_command,
	guild_only,
	custom_data = "CustomData::new(super::pause_help)"
)]
pub async fn pause(ctx: Context<'_>) -> CommandResult {
	run(&ctx, super::pause(&ctx.into(), &(&ctx).into())).await
}

#[poise::command(
	category = "queue",
	prefix_command,
	slash_command,
	guild_only,
	custom_data = "CustomData::new(super::unpause_help)"
)]
pub async fn unpause(ctx: Context<'_>) -> CommandResult {
	run(&ctx, super::unpause(&ctx.into(), &(&ctx).into())).await
}

#[poise::command(
	category = "queue",
	prefix_command,
	slash_command,
	guild_only,
	custom_data = "CustomData::new(super::queue_help)"
)]
pub async fn queue(ctx: Context<'_>) -> CommandResult {
	run(&ctx, super::queue(&ctx.into(), &(&ctx).into())).await
}

async fn shuffle_type_command(ctx: Context<'_>, starting_from: usize) -> CommandResult {
	run(
		&ctx,
		super::shuffle(&ctx.into(), &(&ctx).into(), starting_from),
	)
	.await
}

#[poise::command(
	category = "queue",
	prefix_command,
	slash_command,
	guild_only,
	custom_data = "CustomData::new(super::shuffle_help)"
)]
pub async fn shuffle(ctx: Context<'_>) -> CommandResult {
	shuffle_type_command(ctx, 1).await
}

#[poise::command(
	category = "queue",
	prefix_command,
	slash_command,
	guild_only,
	custom_data = "CustomData::new(super::shufflenow_help)"
)]
pub async fn shufflenow(ctx: Context<'_>) -> CommandResult {
	shuffle_type_command(ctx, 0).await
}

#[poise::command(
	category = "queue",
	rename = "loop",
	prefix_command,
	slash_command,
	guild_only,
	custom_data = "CustomData::new(super::loop_help)"
)]
pub async fn r#loop(
	ctx: Context<'_>,
	#[description = "Number of loops, or \"on\" to loop forever, \"off\" to stop"] count: LoopArg,
) -> CommandResult {
	run(
		&ctx,
		super::r#loop(&ctx.into(), &(&ctx).into(), &LoopArgs { count }),
	)
	.await
}
