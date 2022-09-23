use crate::util::{CommandResult, Context};

use crate::commands::run;

mod generic;

/// Display information about all commands or specific commands
///
/// **Usage:** `help <command?>`
///
/// **Examples:**
/// - `help`
/// - `help play`
#[poise::command(category = "help", prefix_command, slash_command)]
pub async fn help(
	ctx: Context<'_>,
	#[description = "Command to display information about"] command: Vec<String>,
) -> CommandResult {
	run(&ctx, generic::help(&command, ctx.framework())).await
}
