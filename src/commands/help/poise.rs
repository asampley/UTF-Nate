use crate::commands::{run, CustomData};
use crate::util::{CommandResult, Context};

#[poise::command(
	category = "help",
	prefix_command,
	slash_command,
	custom_data = "CustomData { help_md: super::help_help }"
)]
pub async fn help(
	ctx: Context<'_>,
	#[description = "Command to display information about"] command: Vec<String>,
) -> CommandResult {
	run(&ctx, super::help(&command, ctx.framework())).await
}
