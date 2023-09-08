use crate::commands::{run, CustomData};
use crate::util::*;

use super::{CmdArgs, CmdlistArgs};

#[poise::command(
	category = "external",
	prefix_command,
	slash_command,
	custom_data = "CustomData::new(super::cmd_help)"
)]
pub async fn cmd(
	ctx: Context<'_>,
	#[description = "Command to run"] command: String,
	#[description = "Arguments to pass on to the command"] args: Option<String>,
) -> CommandResult {
	run(&ctx, super::cmd(CmdArgs { command, args })).await
}

#[poise::command(
	category = "external",
	prefix_command,
	slash_command,
	custom_data = "CustomData::new(super::cmdlist_help)"
)]
pub async fn cmdlist(
	ctx: Context<'_>,
	#[description = "Path to list commands underneath"] path: Option<String>,
) -> CommandResult {
	run(&ctx, super::cmdlist(&CmdlistArgs { path })).await
}
