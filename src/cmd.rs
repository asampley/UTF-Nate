use serenity::client::Context;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;

use std::path::{Path, PathBuf};
use std::process;
use std::process::Stdio;

use crate::util::*;

fn cmd_path() -> PathBuf {
	return Path::new("./resources/cmd/").canonicalize().unwrap();
}

#[group("external")]
#[description("Commands relating to external commands, such as starting a factorio server")]
#[commands(cmd)]
struct External;

#[command]
#[help_available]
#[description("Execute an external command")]
#[usage("<command> [arg ...]")]
#[example("date")]
pub async fn cmd(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
	let command = match args.single::<String>() {
		Ok(command) => cmd_path().join(&command),
		Err(_) => {
			msg.respond_str(ctx, "Must provide a command").await?;
			return Ok(());
		}
	};

	if !sandboxed_exists(&cmd_path(), &command) {
		msg.respond_str(ctx, "Invalid command").await?;
		return Ok(());
	}

	let message = process::Command::new(command)
		.args(args.iter::<String>().map(|s| s.unwrap()))
		.stdin(Stdio::null())
		.output()
		.map(|o| o.stdout.into_iter().map(|a| a as char).collect::<String>());

	match message {
		Ok(message) => {
			msg.respond_str(ctx, &message).await?;
			println!("Output of command: {}", message);
		}
		Err(reason) => {
			msg.respond_str(ctx, "Error executing command").await?;
			eprintln!("Error executing command: {:?}", reason);
			return Ok(());
		}
	}

	Ok(())
}
