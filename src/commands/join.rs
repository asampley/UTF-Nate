use crate::commands::run;
use crate::util::{CommandResult, Context};

mod generic;

/// Summon the bot to the voice channel the user is currently in
///
/// **Usage:** `summon`
#[poise::command(category = "join", prefix_command, slash_command, guild_only)]
pub async fn summon(ctx: Context<'_>) -> CommandResult {
	run(
		&ctx,
		generic::summon(ctx.discord(), ctx.guild_id(), ctx.author().id),
	)
	.await
}

/// Remove the bot from the voice channel it is in
///
/// **Usage:** `banish`
#[poise::command(category = "join", prefix_command, slash_command, guild_only)]
pub async fn banish(ctx: Context<'_>) -> CommandResult {
	run(&ctx, generic::banish(ctx.discord(), ctx.guild_id())).await
}
