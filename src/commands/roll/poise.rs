use crate::commands::{CustomData, run};
use crate::util::*;

use super::RollArgs;

#[poise::command(
	category = "roll",
	prefix_command,
	slash_command,
	custom_data = "CustomData::new(super::roll_help)"
)]
pub async fn roll(
	ctx: Context<'_>,
	#[description = "Dice expression to roll and calculate"]
	#[rest]
	expression: String,
) -> CommandResult {
	run(&ctx, super::roll(&RollArgs { expression })).await
}
