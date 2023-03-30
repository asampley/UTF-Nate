use crate::commands::run;
use crate::util::{CommandResult, Context};

mod generic;

/// Shuffle a deck of cards an send it back
///
/// **Usage:** `shuffle`
#[poise::command(category = "cards", prefix_command, slash_command, guild_only)]
pub async fn shuffle(ctx: Context<'_>) -> CommandResult {
	run(&ctx, generic::shuffle()).await
}
