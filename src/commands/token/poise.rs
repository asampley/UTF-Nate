use crate::commands::{run, CustomData};
use crate::util::{CommandResult, Context};

#[poise::command(
	category = "http",
	prefix_command,
	slash_command,
	guild_only,
	custom_data = "CustomData { help_md: super::token_help }"
)]
pub async fn token(ctx: Context<'_>) -> CommandResult {
	run(&ctx, super::token((&ctx).into())).await
}
