use crate::commands::{CustomData, run};
use crate::util::{CommandResult, Context};

use super::HelpArgs;

#[poise::command(
	category = "help",
	prefix_command,
	slash_command,
	custom_data = "CustomData::new(super::help_help)"
)]
pub async fn help(
	ctx: Context<'_>,
	#[description = "Command to display information about"] command: Option<String>,
) -> CommandResult {
	run(&ctx, super::help(&HelpArgs { command })).await
}
