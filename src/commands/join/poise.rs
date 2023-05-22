use crate::commands::{run, CustomData};
use crate::util::{CommandResult, Context};

#[poise::command(
	category = "join",
	prefix_command,
	slash_command,
	guild_only,
	custom_data = "CustomData { help_md: super::summon_help }"
)]
pub async fn summon(ctx: Context<'_>) -> CommandResult {
	run(&ctx, super::summon(&ctx.into(), &(&ctx).into())).await
}

#[poise::command(
	category = "join",
	prefix_command,
	slash_command,
	guild_only,
	custom_data = "CustomData { help_md: super::banish_help }"
)]
pub async fn banish(ctx: Context<'_>) -> CommandResult {
	run(&ctx, super::banish(&ctx.into(), &(&ctx).into())).await
}
