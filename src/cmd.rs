use serenity::framework::standard::{Args, Command, CommandError, CommandOptions};
use serenity::model::channel::Message;
use serenity::client::Context;

use std::process;
use std::process::Stdio;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::util::*;

pub struct Cmd;

fn cmd_path() -> PathBuf {
    return Path::new("./resources/cmd/").canonicalize().unwrap();
}

impl Command for Cmd {
    fn options(&self) -> Arc<CommandOptions> {
        Arc::new(CommandOptions {
            help_available: true,
            desc: Some(String::from("Execute an external command")),
            usage: Some(String::from("<command> [arg ...]")),
            example: Some(String::from("date")),
            ..Default::default()
        })
    }

    fn execute(&self, _: &mut Context, msg: &Message, mut args: Args)
        -> Result<(), CommandError>
    {
        let command = match args.single::<String>() {
            Ok(command) => cmd_path().join(&command),
            Err(_) => {
                check_msg(msg.channel_id.say("Must provide a command"));
                return Ok(());
            }
        };

        if !sandboxed_exists(&cmd_path(), &command) {
            check_msg(msg.channel_id.say("Invalid command"));
            return Ok(());
        }

        let message = process::Command::new(command)
            .args(args.iter::<String>().map(|s| s.unwrap()))
            .stdin(Stdio::null())
            .output()
            .map(|o| o.stdout
                .into_iter()
                .map(|a| a as char)
                .collect::<String>()
            );

        match message {
            Ok(message) => {
                check_msg(msg.channel_id.say(&message));
                println!("Output of command: {}", message);
            }
            Err(reason) => {
                check_msg(msg.channel_id.say("Error executing command"));
                eprintln!("Error executing command: {:?}", reason);
                return Ok(());
            }
        }

        Ok(())
    }
}
