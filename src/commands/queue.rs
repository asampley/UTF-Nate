use crate::commands::run;
use crate::parser::Selection;
use crate::util::*;

mod generic;

pub use generic::LoopArg;
pub use generic::ParseLoopArgError;

/// Stop all clips currently being played by the bot
///
/// **Usage:** `stop`
#[poise::command(category = "queue", prefix_command, slash_command, guild_only)]
pub async fn stop(ctx: Context<'_>) -> CommandResult {
	run(&ctx, generic::stop(ctx.discord(), ctx.guild_id())).await
}

/// Skip the current song in the queue, or select a list and/or range of songs to skip
///
/// **Usage:** `skip <selection?>`
///
/// **Examples:**
/// - `skip`
/// - `skip 2,3,4`
/// - `skip 4-7,9,0-2`
#[poise::command(category = "queue", prefix_command, slash_command, guild_only)]
pub async fn skip(
	ctx: Context<'_>,
	#[description = "Range or index of songs to skip, separated by commas"] selection: Option<
		Selection<usize>,
	>,
) -> CommandResult {
	run(
		&ctx,
		generic::skip(ctx.discord(), ctx.guild_id(), selection),
	)
	.await
}

/// Pause the queue
///
/// **Usage:** `pause`
#[poise::command(category = "queue", prefix_command, slash_command, guild_only)]
pub async fn pause(ctx: Context<'_>) -> CommandResult {
	run(&ctx, generic::pause(ctx.discord(), ctx.guild_id())).await
}

/// Unpause the queue
///
/// **Usage:** `unpause`
#[poise::command(category = "queue", prefix_command, slash_command, guild_only)]
pub async fn unpause(ctx: Context<'_>) -> CommandResult {
	run(&ctx, generic::unpause(ctx.discord(), ctx.guild_id())).await
}

/// Show the current queue of songs
///
/// **Usage:** `queue`
#[poise::command(category = "queue", prefix_command, slash_command, guild_only)]
pub async fn queue(ctx: Context<'_>) -> CommandResult {
	run(&ctx, generic::queue(ctx.discord(), ctx.guild_id())).await
}

async fn shuffle_type_command(ctx: Context<'_>, starting_from: usize) -> CommandResult {
	run(
		&ctx,
		generic::shuffle(ctx.discord(), ctx.guild_id(), starting_from),
	)
	.await
}

/// Shuffle the queue of songs, after the current song
///
/// **Usage:** `shuffle`
#[poise::command(category = "queue", prefix_command, slash_command, guild_only)]
pub async fn shuffle(ctx: Context<'_>) -> CommandResult {
	shuffle_type_command(ctx, 1).await
}

/// Shuffle the queue of songs, including the current song
///
/// **Usage:** `shufflenow`
#[poise::command(category = "queue", prefix_command, slash_command, guild_only)]
pub async fn shufflenow(ctx: Context<'_>) -> CommandResult {
	shuffle_type_command(ctx, 0).await
}

/// Set how many time the current song should loop
///
/// **Usage:** `loop <loop times>`
///
/// **Examples:**
/// - `loop on`
/// - `loop off`
/// - `loop 3`
#[poise::command(
	category = "queue",
	rename = "loop",
	prefix_command,
	slash_command,
	guild_only
)]
pub async fn r#loop(
	ctx: Context<'_>,
	#[description = "Number of loops, or \"on\" to loop forever, \"off\" to stop"] count: LoopArg,
) -> CommandResult {
	run(&ctx, generic::r#loop(ctx.discord(), ctx.guild_id(), count)).await
}
