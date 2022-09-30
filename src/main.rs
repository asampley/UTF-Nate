mod audio;
mod commands;
mod configuration;
mod data;
mod handler;
mod interaction;
mod parser;
mod spotify;
mod util;
mod youtube;

use clap::Parser;

use tracing::{error, info};
use tracing_subscriber::filter::LevelFilter;

use once_cell::sync::Lazy;

use serenity::http::client::Http;
use serenity::model::gateway::GatewayIntents;
use serenity::model::Permissions;
use serenity::prelude::RwLock;

use songbird::serenity::SerenityInit;

use sqlx::{Executor, PgPool};

use configuration::Config;
use data::{Keys, VoiceGuilds, VoiceUserCache};
use handler::Handler;
use interaction::reregister;
use util::{check_msg, read_toml, Context, FrameworkError, Respond};

use std::fmt::{Debug, Write};
use std::path::Path;
use std::sync::Arc;

const RESOURCE_PATH: Lazy<&'static Path> = Lazy::new(|| Path::new("resources/"));

static OPT: Lazy<Opt> = Lazy::new(|| {
	let opt = Opt::parse();
	println!("Options: {:#?}", opt);
	opt
});

static CONFIG: Lazy<Config> = Lazy::new(|| load_config());

const RECOMMENDED_PERMISSIONS: Permissions = Permissions::SEND_MESSAGES
	.union(Permissions::EMBED_LINKS)
	.union(Permissions::CONNECT)
	.union(Permissions::SPEAK);

const GATEWAY_INTENTS: GatewayIntents = GatewayIntents::GUILD_MESSAGES
	.union(GatewayIntents::DIRECT_MESSAGES)
	.union(GatewayIntents::GUILD_VOICE_STATES)
	.union(GatewayIntents::GUILDS)
	.union(GatewayIntents::MESSAGE_CONTENT);

struct Pool;

impl serenity::prelude::TypeMapKey for Pool {
	type Value = PgPool;
}

#[derive(Debug, Parser)]
struct Opt {
	/// Run initializing scripts for database
	#[clap(long)]
	init_database: bool,

	/// Reregister slash commands with discord
	#[clap(long)]
	reregister: bool,

	/// Do not run the bot. Useful when registering slash commands or initializing the database
	#[clap(long)]
	no_bot: bool,

	/// Run command with additional logging
	#[clap(long, short)]
	verbose: bool,

	/// Do not check for clip collisions. Speeds up start by disabling.
	#[clap(long)]
	no_check_clips: bool,
}

#[tokio::main]
async fn main() {
	// initialize logging
	let subscriber = tracing_subscriber::fmt()
		.with_max_level(match OPT.verbose {
			true => LevelFilter::DEBUG,
			false => LevelFilter::INFO,
		})
		.compact()
		.finish();

	tracing::subscriber::set_global_default(subscriber)
		.expect("unable to set default tracing subscriber");

	if !OPT.no_check_clips {
		// warn if there are duplicate clip names
		audio::warn_duplicate_clip_names();
		// warn if clips cannot be found with search easily
		audio::warn_exact_name_finds_different_clip();
	}

	// read keys file
	let keys_path = "keys.toml";
	let keys: Keys = match read_toml(keys_path) {
		Ok(k) => k,
		Err(e) => {
			error!("Error reading keys file {keys_path:?}: {e}");
			return;
		}
	};

	// print recommended permissions invite URL
	info!(
		"Add the bot using the url:\n\
		https://discord.com/api/oauth2/authorize?client_id={}&permissions={}&scope=bot%20applications.commands",
		keys.discord.application_id,
		RECOMMENDED_PERMISSIONS.bits(),
	);

	// print recommended permissions invite URL without slash commands
	info!(
		"To disallow slash commands, use this url instead:\n\
		https://discord.com/api/oauth2/authorize?client_id={}&permissions={}&scope=bot",
		keys.discord.application_id,
		RECOMMENDED_PERMISSIONS.bits(),
	);

	let http = Http::new_with_application_id(&keys.discord.token, keys.discord.application_id);

	let commands = commands::commands();

	if OPT.reregister {
		match reregister(&http, &commands).await {
			Ok(()) => (),
			Err(e) => {
				error!("Unable to reregister slash commands: {e}");
				return;
			}
		}
	}

	if OPT.init_database || !OPT.no_bot {
		// initialize database connection
		let db_pool = match PgPool::connect(&keys.database.connect_string).await {
			Ok(p) => p,
			Err(e) => {
				error!("Failed to connect to database: {e}");
				return;
			}
		};

		if OPT.init_database {
			let create_tables = match std::fs::read_to_string("database/create-tables.sql") {
				Ok(t) => t,
				Err(e) => {
					error!("Failed to read create tables file: {e}");
					return;
				}
			};

			let mut trans = match db_pool.begin().await {
				Ok(t) => t,
				Err(e) => {
					error!("Failed to intialize database transaction: {e}");
					return;
				}
			};

			match trans.execute(create_tables.as_str()).await {
				Ok(_) => (),
				Err(e) => {
					error!("Error creating tables: {e}");
					return;
				}
			}

			match trans.commit().await {
				Ok(_) => (),
				Err(e) => {
					error!("Error committing table creation: {e}");
					return;
				}
			}

			info!("Data tables created");
		}

		if !OPT.no_bot {
			info!("Config: {CONFIG:#?}");

			// create a framework to process message commands
			poise::Framework::builder()
				.token(&keys.discord.token)
				.intents(GATEWAY_INTENTS)
				.user_data_setup(|_, _, _| Box::pin(async move { Ok(()) }))
				.options(poise::FrameworkOptions {
					prefix_options: poise::PrefixFrameworkOptions {
						prefix: Some(CONFIG.prefixes[0].clone()),
						additional_prefixes: CONFIG.prefixes[1..]
							.iter()
							.map(|p| poise::Prefix::Literal(p))
							.collect(),
						case_insensitive_commands: true,
						..Default::default()
					},
					commands: commands,
					pre_command: |ctx| Box::pin(before_hook(ctx)),
					post_command: |ctx| Box::pin(after_hook(ctx)),
					on_error: |err| Box::pin(on_error(err)),
					..Default::default()
				})
				.client_settings(|client_builder| {
					client_builder
						.event_handler(Handler)
						.type_map_insert::<VoiceUserCache>(Default::default())
						.type_map_insert::<VoiceGuilds>(Default::default())
						.type_map_insert::<Keys>(Arc::new(RwLock::new(keys)))
						.type_map_insert::<Pool>(db_pool)
						.register_songbird_from_config(
							songbird::Config::default()
								.decode_mode(songbird::driver::DecodeMode::Pass)
								.preallocated_tracks(5),
						)
				})
				.run()
				.await
				.expect("Error starting bot");
			}
		}
	}

fn load_config() -> Config {
	let path = "config.toml";

	match read_toml(path) {
		Ok(config) => {
			info!("Read config file from {path:?}");
			config
		}
		Err(e) => {
			error!("{e}");
			info!("Creating default config");
			Config::default()
		}
	}
}

async fn before_hook(ctx: Context<'_>) {
	let guild_name = ctx
		.guild_id()
		.map(|gid| ctx.discord().cache.guild_field(gid, |g| g.name.clone()));

	info!(
		"User {} ({}) in guild {:?} ({:?}) running {}",
		ctx.author().name,
		ctx.author().id,
		guild_name,
		ctx.guild_id(),
		ctx.invoked_command_name()
	);
}

async fn after_hook(ctx: Context<'_>) {
	let guild_name = ctx
		.guild_id()
		.map(|gid| ctx.discord().cache.guild_field(gid, |g| g.name.clone()));

	info!(
		"User {} ({}) in guild {:?} ({:?}) completed {}",
		ctx.author().name,
		ctx.author().id,
		guild_name,
		ctx.guild_id(),
		ctx.invoked_command_name()
	);
}

async fn on_error(err: FrameworkError<'_>) {
	use poise::FrameworkError::*;

	match &err {
		GuildOnly { ctx } | DmOnly { ctx } | NsfwOnly { ctx } => {
			check_msg(
				ctx.respond_err(
					&format!(
						"`{}{}` is only available in {}",
						ctx.prefix(),
						ctx.command().qualified_name,
						match &err {
							GuildOnly { .. } => "guilds",
							DmOnly { .. } => "dms",
							NsfwOnly { .. } => "nsfw channels",
							_ => unreachable!(),
						}
					)
					.into(),
				)
				.await,
			);
		}
		ArgumentParse { error, ctx, .. } => {
			let mut response = if error.is::<poise::TooManyArguments>() {
				format!("Too many arguments supplied")
			} else if error.is::<poise::TooFewArguments>() {
				format!("Too few arguments supplied")
			} else if error.is::<core::num::ParseFloatError>() {
				format!("Expected a float like 0.5")
			} else if error.is::<commands::queue::ParseLoopArgError>()
				|| error.is::<parser::ParseSelectionError>()
			{
				let mut msg = format!("{}.", error);
				msg[..1].make_ascii_uppercase();
				msg
			} else {
				error!("Unhandled argument parse error: {:?}", error);

				format!("Could not parse arguments for command")
			};

			write!(
				response,
				"\n\nUse `{}help {}` for more info",
				ctx.prefix(),
				ctx.command().qualified_name
			)
			.unwrap();

			check_msg(ctx.respond_err(&response.into()).await);
		}
		_ => error!("Unhandled error: {:?}", err),
	}
}
