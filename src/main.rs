mod cmd;
mod configuration;
mod data;
mod handler;
mod unicode;
#[macro_use] mod util;
mod voice;
mod herald;

use once_cell::sync::Lazy;

use serenity::client::Client;
use serenity::framework::standard::macros::{help, hook};
use serenity::framework::standard::{
	help_commands, Args, CommandGroup, CommandResult, DispatchError, HelpOptions, StandardFramework,
};
use serenity::model::channel::Message;
use serenity::model::id::UserId;
use serenity::prelude::Context;
use serenity::prelude::RwLock;

use songbird::serenity::SerenityInit;

use structopt::StructOpt;

use cmd::EXTERNAL_GROUP;
use configuration::{read_config, Config};
use data::{VoiceGuilds, VoiceUserCache};
use handler::Handler;
use herald::HERALD_GROUP;
use unicode::UNICODE_GROUP;
use util::{Respond, check_msg};
use voice::VOICE_GROUP;

use std::collections::HashSet;
use std::path::Path;
use std::sync::Arc;

#[derive(Debug, StructOpt)]
struct Opt {
	#[structopt(long, help = "Reregister slash commands with discord")]
	reregister: bool,

	#[structopt(long, short, help = "Run command with additional logging")]
	verbose: bool,
}

static OPT: Lazy<Opt> = Lazy::new(|| {
	let opt = Opt::from_args();
	println!("Options: {:#?}", opt);
	opt
});

#[tokio::main]
async fn main() {
	// login with a bot token from file
	let mut client = Client::builder(&read_token().expect("Token could not be read"))
		.application_id(
			read_application_id()
				.expect("Application id could not be read")
				.trim().parse()
				.expect("Application id could not be parsed")
		).event_handler(Handler)
		.framework(
			// create a framework to process message commands
			StandardFramework::new()
				.configure(|c| c.prefix("!"))
				.before(before_hook)
				.after(after_hook)
				.help(&HELP)
				.group(&UNICODE_GROUP)
				.group(&VOICE_GROUP)
				.group(&HERALD_GROUP)
				.group(&EXTERNAL_GROUP)
				.unrecognised_command(unrecognised_command)
				.on_dispatch_error(on_dispatch_error),
		)
		.type_map_insert::<VoiceUserCache>(Default::default())
		.type_map_insert::<VoiceGuilds>(Default::default())
		.type_map_insert::<Config>(Arc::new(RwLock::new(load_config())))
		.register_songbird()
		.await
		.expect("Error creating client");

	if let Err(reason) = client.start().await {
		eprintln!("An error occurred while running the client: {:?}", reason)
	}
}

fn read_token() -> std::io::Result<String> {
	std::fs::read_to_string("token")
}

fn read_application_id() -> std::io::Result<String> {
	std::fs::read_to_string("application_id")
}

fn load_config() -> Config {
	use configuration::Result::*;

	match read_config(Path::new("config.json")) {
		Ok(config) => {
			println!("Read config file from config.json");
			config
		}
		JsonError(reason) => {
			eprintln!("Error parsing config.json: {:?}", reason);
			println!("Creating default config");
			Config::default()
		}
		IoError(reason) => {
			eprintln!("Unable to access config.json: {:?}", reason);
			println!("Creating default config");
			Config::default()
		}
	}
}

#[help]
async fn help(
	ctx: &Context,
	msg: &Message,
	args: Args,
	help_options: &'static HelpOptions,
	groups: &[&'static CommandGroup],
	owners: HashSet<UserId>,
) -> CommandResult {
	help_commands::plain(ctx, msg, args, help_options, groups, owners).await;
	Ok(())
}

#[hook]
async fn unrecognised_command(ctx: &Context, msg: &Message, cmd: &str) {
	let guild_name = msg.guild_field(&ctx.cache, |g| g.name.clone()).await;
	check_msg(msg.reply(&ctx, format!("Unrecognised command: {}", cmd)).await);

	println!(
		"User {} ({}) in guild {:?} ({:?}) command {} not recognised with message: {}",
		msg.author.name, msg.author.id, guild_name, msg.guild_id, cmd, msg.content
	);
}

#[hook]
async fn before_hook(ctx: &Context, msg: &Message, cmd: &str) -> bool {
	let guild_name = msg.guild_field(&ctx.cache, |g| g.name.clone()).await;
	println!(
		"User {} ({}) in guild {:?} ({:?}) running {} with message: {}",
		msg.author.name, msg.author.id, guild_name, msg.guild_id, cmd, msg.content
	);

	true
}

#[hook]
async fn after_hook(ctx: &Context, msg: &Message, cmd: &str, res: CommandResult) {
	let guild_name = msg.guild_field(&ctx.cache, |g| g.name.clone()).await;

	println!(
		"User {} ({}) in guild {:?} ({:?}) completed {} with result {:?} with message: {}",
		msg.author.name, msg.author.id, guild_name, msg.guild_id, cmd, res, msg.content
	);
}

#[hook]
async fn on_dispatch_error(ctx: &Context, msg: &Message, err: DispatchError) {
	use DispatchError::*;
	match err {
		NotEnoughArguments { min, given } => {
			let s = format!(
				"Too few arguments. Expected at least {}, but got {}.",
				min, given
			);

			check_msg(msg.respond_str(ctx, &s).await);
		}
		TooManyArguments { max, given } => {
			let s = format!(
				"Too many arguments. Expected at most {}, but got {}.",
				max, given
			);

			check_msg(msg.respond_str(ctx, &s).await);
		}
		OnlyForGuilds => {
			check_msg(msg.respond_str(ctx, "This command is only available in guilds").await);
		}
		_ => println!("Unhandled dispatch error: {:?}", err),
	}
}
