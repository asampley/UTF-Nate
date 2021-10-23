use log::{error, info};

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
			msg.respond_err(ctx, &"Must provide a command".into())
				.await?;
			return Ok(());
		}
	};

	if !sandboxed_exists(&cmd_path(), &command) {
		msg.respond_err(ctx, &"Invalid command".into()).await?;
		return Ok(());
	}

	let message = process::Command::new(&command)
		.args(args.iter::<String>().map(|s| s.unwrap()))
		.stdin(Stdio::null())
		.output()
		.map(|o| o.stdout.into_iter().map(|a| a as char).collect::<String>());

	match message {
		Ok(message) => {
			info!("Output of command: {}", message);
			msg.respond_ok(ctx, &message.into()).await?;
		}
		Err(reason) => {
			msg.respond_err(ctx, &"Error executing command".into())
				.await?;
			error!("Error executing command {:?}: {:?}", command, reason);
			return Ok(());
		}
	}

	Ok(())
}
