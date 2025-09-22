use crate::commands::{CustomData, run};
use crate::util::{CommandResult, Context};

#[poise::command(
	category = "http",
	slash_command,
	ephemeral,
	custom_data = "CustomData::new(super::token_help)"
)]
pub async fn token(ctx: Context<'_>) -> CommandResult {
	run(&ctx, super::token(&ctx.into(), &(&ctx).into())).await
}
