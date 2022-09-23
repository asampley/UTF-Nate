use std::path::{Path, PathBuf};

use crate::commands::run;
use crate::util::*;

mod generic;

fn cmd_path() -> PathBuf {
	return Path::new("./resources/cmd/").canonicalize().unwrap();
}

/// Execute an external command
///
/// **Usage:** `cmd <command> [arg ...]`
///
/// **Examples:**
/// - `cmd date`
#[poise::command(category = "external", prefix_command, slash_command)]
pub async fn cmd(
	ctx: Context<'_>,
	#[description = "Command to run"] command: String,
	#[description = "Arguments to pass on to the command"] args: Vec<String>,
) -> CommandResult {
	run(
		&ctx,
		generic::cmd(&command, args.iter().map(|a| a.as_str())),
	)
	.await
}

/// List available commands to be run with cmd
///
/// **Usage:** `cmdlist <section?>`
///
/// **Examples:**
/// - `cmdlist`
/// - `cmdlist valheim`
#[poise::command(category = "external", prefix_command, slash_command)]
pub async fn cmdlist(
	ctx: Context<'_>,
	#[description = "Path to list commands underneath"] path: Option<String>,
) -> CommandResult {
	run(&ctx, generic::cmdlist(path.as_deref())).await
}
