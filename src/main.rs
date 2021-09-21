mod audio;
mod commands;
mod configuration;
mod data;
mod handler;
mod spotify;
mod util;
mod youtube;

use log::{error, info, LevelFilter};

use once_cell::sync::Lazy;

use serenity::client::bridge::gateway::GatewayIntents;
use serenity::client::Client;
use serenity::framework::standard::macros::hook;
use serenity::framework::standard::{
	CommandGroup, CommandResult, DispatchError, StandardFramework,
};
use serenity::model::channel::Message;
use serenity::prelude::{Context, RwLock};

use songbird::serenity::SerenityInit;

use structopt::StructOpt;

use configuration::{read_config, Config};
use data::{Keys, VoiceGuilds, VoiceUserCache};
use handler::Handler;
use util::{check_msg, Respond};

use std::path::Path;
use std::sync::Arc;

static PREFIXES: &[&'static str] = &["!"];

static OPT: Lazy<Opt> = Lazy::new(|| {
	let opt = Opt::from_args();
	println!("Options: {:#?}", opt);
	opt
});

static GROUPS: &[&'static CommandGroup] = &[
	&commands::help::HELP_GROUP,
	&commands::herald::HERALD_GROUP,
	&commands::join::JOIN_GROUP,
	&commands::play::PLAY_GROUP,
	&commands::queue::QUEUE_GROUP,
	&commands::unicode::UNICODE_GROUP,
	&commands::voice::VOICE_GROUP,
	&commands::cmd::EXTERNAL_GROUP,
];

#[derive(Debug, StructOpt)]
struct Opt {
	#[structopt(long, help = "Reregister slash commands with discord")]
	reregister: bool,

	#[structopt(long, short, help = "Run command with additional logging")]
	verbose: bool,
}

#[tokio::main]
async fn main() {
	// initialize logging
	env_logger::Builder::new()
		.filter_module(
			"utf_nate",
			match OPT.verbose {
				true => LevelFilter::Debug,
				false => LevelFilter::Info,
			},
		)
		.filter_module(
			"songbird",
			match OPT.verbose {
				true => LevelFilter::Debug,
				false => LevelFilter::Info,
			},
		)
		.format_timestamp_micros()
		.init();

	// warn if there are duplicate clip names
	audio::warn_duplicate_clip_names();

	let keys = serde_json::from_str::<Keys>(
		&std::fs::read_to_string("keys.json").expect("Unable to read keys file"),
	)
	.expect("Unable to parse keys file");

	// create a framework to process message commands
	let framework = StandardFramework::new()
		.configure(|c| c.prefixes(PREFIXES))
		.before(before_hook)
		.after(after_hook)
		.unrecognised_command(unrecognised_command)
		.on_dispatch_error(on_dispatch_error);

	let framework = GROUPS.iter().fold(framework, |f, group| f.group(group));

	// login with a bot token from file
	let mut client = Client::builder(&keys.token)
		.application_id(keys.application_id)
		.intents(
			GatewayIntents::GUILD_MESSAGES
				| GatewayIntents::DIRECT_MESSAGES
				| GatewayIntents::GUILD_VOICE_STATES
				| GatewayIntents::GUILDS,
		)
		.event_handler(Handler)
		.framework(framework)
		.type_map_insert::<VoiceUserCache>(Default::default())
		.type_map_insert::<VoiceGuilds>(Default::default())
		.type_map_insert::<Config>(Arc::new(RwLock::new(load_config())))
		.type_map_insert::<Keys>(Arc::new(RwLock::new(keys)))
		.register_songbird_from_config(
			songbird::Config::default()
				.decode_mode(songbird::driver::DecodeMode::Pass)
				.preallocated_tracks(5),
		)
		.await
		.expect("Error creating client");

	if let Err(reason) = client.start().await {
		error!("An error occurred while running the client: {:?}", reason)
	}
}

fn load_config() -> Config {
	use crate::util::JsonFileError::*;

	match read_config(Path::new("config.json")) {
		Ok(config) => {
			info!("Read config file from config.json");
			config
		}
		Err(e) => match e {
			JsonError(reason) => {
				error!("Error parsing config.json: {:?}", reason);
				info!("Creating default config");
				Config::default()
			}
			IoError(reason) => {
				error!("Unable to access config.json: {:?}", reason);
				info!("Creating default config");
				Config::default()
			}
		},
	}
}

#[hook]
async fn unrecognised_command(ctx: &Context, msg: &Message, cmd: &str) {
	let guild_name = msg.guild_field(&ctx.cache, |g| g.name.clone()).await;
	check_msg(
		msg.reply(&ctx, format!("Unrecognised command: {}", cmd))
			.await,
	);

	info!(
		"User {} ({}) in guild {:?} ({:?}) command {} not recognised with message: {}",
		msg.author.name, msg.author.id, guild_name, msg.guild_id, cmd, msg.content
	);
}

#[hook]
async fn before_hook(ctx: &Context, msg: &Message, cmd: &str) -> bool {
	let guild_name = msg.guild_field(&ctx.cache, |g| g.name.clone()).await;
	info!(
		"User {} ({}) in guild {:?} ({:?}) running {} with message: {}",
		msg.author.name, msg.author.id, guild_name, msg.guild_id, cmd, msg.content
	);

	true
}

#[hook]
async fn after_hook(ctx: &Context, msg: &Message, cmd: &str, res: CommandResult) {
	let guild_name = msg.guild_field(&ctx.cache, |g| g.name.clone()).await;

	info!(
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

			check_msg(msg.respond_err(ctx, &s.into()).await);
		}
		TooManyArguments { max, given } => {
			let s = format!(
				"Too many arguments. Expected at most {}, but got {}.",
				max, given
			);

			check_msg(msg.respond_err(ctx, &s.into()).await);
		}
		OnlyForGuilds => {
			check_msg(
				msg.respond_err(ctx, &"This command is only available in guilds".into())
					.await,
			);
		}
		_ => error!("Unhandled dispatch error: {:?}", err),
	}
}
