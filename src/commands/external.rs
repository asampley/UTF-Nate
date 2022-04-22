use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{Args, CommandResult, Delimiter};
use serenity::model::channel::Message;
use serenity::model::interactions::application_command::{
	ApplicationCommandInteraction, ApplicationCommandOptionType,
};

use std::path::{Path, PathBuf};

use crate::commands::{create_interaction, run};
use crate::util::*;

mod generic;

fn cmd_path() -> PathBuf {
	return Path::new("./resources/cmd/").canonicalize().unwrap();
}

#[group("external")]
#[description("Commands relating to external commands, such as starting a factorio server")]
#[commands(cmd, cmdlist)]
struct External;

#[command]
#[help_available]
#[description("Execute an external command")]
#[usage("<command> [arg ...]")]
#[example("date")]
pub async fn cmd(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	let mut args = args.raw_quoted();
	let command = args.next();

	run(ctx, msg, generic::cmd(command, args)).await
}

pub async fn cmd_interaction(
	ctx: &Context,
	int: &ApplicationCommandInteraction,
) -> serenity::Result<()> {
	let command = match get_option_string(ctx, int, &int.data.options, "command").await {
		Ok(value) => value,
		Err(result) => return result,
	};

	let args = match get_option_string(ctx, int, &int.data.options, "command").await {
		Ok(value) => value,
		Err(result) => return result,
	}
	.unwrap_or("");

	let args = Args::new(args, &[Delimiter::Single(' ')]);
	let args = args.raw_quoted();

	run(ctx, int, generic::cmd(command, args)).await
}

pub fn cmd_interaction_create(cmd: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
	create_interaction(&CMD_COMMAND, cmd)
		.create_option(|option| {
			option
				.name("command")
				.description("Path to list commands underneath")
				.kind(ApplicationCommandOptionType::String)
				.required(true)
		})
		.create_option(|option| {
			option
				.name("args")
				.description("Arguments to pass on to the command")
				.kind(ApplicationCommandOptionType::String)
		})
}

#[command]
#[help_available]
#[description("List available commands to be run with cmd")]
#[min_args(0)]
#[max_args(1)]
#[usage("<section?>")]
#[example("")]
#[example("valheim")]
pub async fn cmdlist(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	if args.len() > 1 {
		msg.respond_err(ctx, &"Expected at most one path to be specified".into())
			.await?;
		return Ok(());
	}

	let path = args.current();

	run(ctx, msg, generic::cmdlist(path)).await
}

pub async fn cmdlist_interaction(
	ctx: &Context,
	int: &ApplicationCommandInteraction,
) -> serenity::Result<()> {
	let path = match get_option_string(ctx, int, &int.data.options, "path").await {
		Ok(value) => value,
		Err(result) => return result,
	};

	run(ctx, int, generic::cmdlist(path)).await
}

pub fn cmdlist_interaction_create(
	cmd: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
	create_interaction(&CMDLIST_COMMAND, cmd).create_option(|option| {
		option
			.name("path")
			.description("Path to list commands underneath")
			.kind(ApplicationCommandOptionType::String)
	})
}
