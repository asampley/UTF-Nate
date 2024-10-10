#![feature(try_blocks)]
#![cfg_attr(test, feature(assert_matches))]

mod audio;
mod commands;
mod configuration;
mod data;
#[cfg(feature = "http-interface")]
mod encrypt;
mod handler;
#[cfg(feature = "http-interface")]
mod http;
mod interaction;
mod parser;
mod persistence;
mod spotify;
mod util;
mod youtube;

use clap::Parser;

use persistence::StorageError;
use ring::aead::LessSafeKey;

use tap::TapFallible;
use thiserror::Error;

use tokio::task::JoinSet;

use tracing::{error, info};
use tracing_subscriber::filter::LevelFilter;

use once_cell::sync::Lazy;

use serenity::client::Client;
use serenity::http::Http;
use serenity::model::gateway::GatewayIntents;
use serenity::model::Permissions;
use serenity::prelude::RwLock;

use songbird::serenity::SerenityInit;

use configuration::Config;
use data::{Keys, VoiceGuilds, VoiceUserCache};
use handler::Handler;
use interaction::reregister;
use util::{read_toml, Framework};

use std::fmt::Debug;
use std::path::Path;
use std::sync::Arc;

use crate::persistence::Storage;

/// Path to shared resources directory for things such as clips or database scripts.
static RESOURCE_PATH: Lazy<&'static Path> = Lazy::new(|| Path::new("resources/"));

/// Options parsed from the command line using [`clap`].
static OPT: Lazy<Opt> = Lazy::new(|| {
	let opt = Opt::parse();
	println!("Options: {:#?}", opt);
	opt
});

/// Configuration parameters from a file. See [`load_config()`].
static CONFIG: Lazy<Config> = Lazy::new(load_config);

static REQWEST_CLIENT: Lazy<reqwest::Client> = Lazy::new(reqwest::Client::new);

/// Permissions recommended for registering the bot with a server, for full
/// functionality.
///
/// If some permissions are excluded when adding the bot to a server, it may not
/// function properly.
const RECOMMENDED_PERMISSIONS: Permissions = Permissions::SEND_MESSAGES
	.union(Permissions::EMBED_LINKS)
	.union(Permissions::CONNECT)
	.union(Permissions::SPEAK);

/// Gateway intents registered with discord to properly receive events from
/// discord's API.
const GATEWAY_INTENTS: GatewayIntents = GatewayIntents::GUILD_MESSAGES
	.union(GatewayIntents::DIRECT_MESSAGES)
	.union(GatewayIntents::GUILD_VOICE_STATES)
	.union(GatewayIntents::GUILDS)
	.union(GatewayIntents::MESSAGE_CONTENT);

/// Key for persistent storage resource.
struct StorageKey;

impl serenity::prelude::TypeMapKey for StorageKey {
	type Value = sqlx::Pool<sqlx::Any>;
}

/// Key for [`ring::aead::LessSafeKey`] for encryption purposes.
#[allow(dead_code)]
enum AeadKey {}

impl serenity::prelude::TypeMapKey for AeadKey {
	type Value = LessSafeKey;
}

#[derive(Debug, Parser)]
struct Opt {
	/// Run initializing scripts for database.
	#[arg(long)]
	init_database: bool,

	/// Reregister slash commands with discord.
	#[arg(long)]
	reregister: bool,

	/// Do not run the bot. Useful when registering slash commands or
	/// initializing the database.
	#[arg(long)]
	no_bot: bool,

	/// Run command with additional logging.
	#[arg(long, short)]
	verbose: bool,

	/// Do not check for clip collisions. Speeds up start by disabling.
	#[arg(long)]
	no_check_clips: bool,
}

#[derive(Debug, Error)]
enum ProcessError {
	#[error(transparent)]
	Serenity(#[from] serenity::prelude::SerenityError),

	#[cfg(feature = "http-interface")]
	#[error(transparent)]
	Hyper(#[from] hyper::Error),
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

	if let Err(e) = tracing::subscriber::set_global_default(subscriber) {
		error!("Unable to set default tracing subscriber: {:?}", e);
		return;
	}

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

	let http = Http::new(&keys.discord.token);
	http.set_application_id(keys.discord.application_id.into());

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

	sqlx::any::install_default_drivers();

	if OPT.init_database || !OPT.no_bot {
		// initialize database connection
		let db_pool = match init_database(&keys.database.connect_string, OPT.init_database).await {
			Ok(v) => v,
			Err(e) => {
				error!("Error initializing database: {e}");
				return;
			}
		};

		if !OPT.no_bot {
			let mut join_set = JoinSet::<Result<(), ProcessError>>::new();

			info!("Config: {:#?}", *CONFIG);

			// create a framework to process message commands
			let client_builder = Client::builder(&keys.discord.token, GATEWAY_INTENTS)
				.event_handler(Handler::default())
				.type_map_insert::<VoiceUserCache>(Default::default())
				.type_map_insert::<VoiceGuilds>(Default::default())
				.type_map_insert::<Keys>(Arc::new(RwLock::new(keys)))
				.type_map_insert::<StorageKey>(db_pool)
				.register_songbird_from_config(songbird::Config::default().preallocated_tracks(5))
				.framework(
					Framework::builder()
						.setup(|_, _, _| Box::pin(async move { Ok(()) }))
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
							commands,
							pre_command: |ctx| Box::pin(handler::before_hook(ctx)),
							post_command: |ctx| Box::pin(handler::after_hook(ctx)),
							on_error: |err| Box::pin(handler::on_error(err)),
							..Default::default()
						})
						.build(),
				);

			#[cfg(feature = "http-interface")]
			let client_builder = client_builder.type_map_insert::<AeadKey>(encrypt::gen_key());

			let mut client = match client_builder.await {
				Ok(client) => client,
				Err(e) => {
					error!("Error starting bot: {:?}", e);
					return;
				}
			};

			#[cfg(feature = "http-interface")]
			if let Some(addr) = &CONFIG.http {
				use axum::routing::*;

				use tower_http::services::ServeDir;

				use crate::commands::http::form_endpoint;
				use crate::commands::http::FormRouter;
				use crate::commands::*;

				let state = commands::BotState {
					data: client.data.clone(),
					cache: client.cache.clone(),
					http: client.http.clone(),
				};

				info!("Starting HTTP server");

				let app = axum::Router::new()
					.form_route(external::poise::cmd, external::http::cmd)
					.form_route(external::poise::cmdlist, external::http::cmdlist)
					.form_route(join::poise::summon, join::http::summon)
					.form_route(join::poise::banish, join::http::banish)
					.form_route(herald::poise::intro, herald::http::intro)
					.form_route(herald::poise::introbot, herald::http::introbot)
					.form_route(herald::poise::outro, herald::http::outro)
					.form_route(play::poise::clip, play::http::clip)
					.form_route(play::poise::play, play::http::play)
					.form_route(play::poise::playnext, play::http::playnext)
					.form_route(play::poise::playnow, play::http::playnow)
					.form_route(queue::poise::stop, queue::http::stop)
					.form_route(queue::poise::skip, queue::http::skip)
					.form_route(queue::poise::pause, queue::http::pause)
					.form_route(queue::poise::unpause, queue::http::unpause)
					.form_route(queue::poise::queue, queue::http::queue)
					.form_route(queue::poise::shuffle, queue::http::shuffle)
					.form_route(queue::poise::shufflenow, queue::http::shufflenow)
					.form_route(queue::poise::r#loop, queue::http::r#loop)
					.form_route(queue::poise::r#move, queue::http::r#move)
					.route(
						"/volume/get",
						get(|| async { form_endpoint(voice::poise::volume_get) }),
					)
					.route("/volume/get/run", get(voice::http::volume_get))
					.route(
						"/volume/clip",
						get(|| async { form_endpoint(voice::poise::volume_clip) }),
					)
					.route("/volume/clip/run", get(voice::http::volume_clip))
					.route(
						"/volume/play",
						get(|| async { form_endpoint(voice::poise::volume_play) }),
					)
					.route("/volume/play/run", get(voice::http::volume_play))
					.route(
						"/volume/now",
						get(|| async { form_endpoint(voice::poise::volume_now) }),
					)
					.route("/volume/now/run", get(voice::http::volume_now))
					.form_route(unicode::poise::unicode, unicode::http::unicode)
					.form_route(roll::poise::roll, roll::http::roll)
					.route("/token", get(token::http::token))
					.fallback_service(ServeDir::new("resources/web"))
					.with_state(state);

				let http_future = hyper::Server::bind(addr).serve(app.into_make_service());

				join_set.spawn(async move { http_future.await.map_err(Into::into) });
			}

			join_set.spawn(async move { client.start().await.map_err(Into::into) });

			while let Some(res) = join_set.join_next().await {
				match res {
					Ok(res) => match res {
						Ok(_) => (),
						Err(e) => {
							error!("Failure in joined process: {:?}", e);
							return;
						}
					},
					Err(e) => error!("Failed to join: {}", e),
				}
			}
		}
	}
}

/// Load the configuration from `config.toml`.
fn load_config() -> Config {
	let path = "config.toml";

	read_toml(path)
		.tap_ok(|_| info!("Read config file from {:?}", path))
		.tap_err(|e| error!("{:?}", e))
		.unwrap_or_else(|_| {
			info!("Creating default config");
			Config::default()
		})
}

async fn init_database(
	connect_string: &str,
	create_tables: bool,
) -> Result<sqlx::Pool<sqlx::Any>, StorageError> {
	let db_pool = sqlx::Pool::<sqlx::Any>::connect(connect_string).await?;

	if create_tables {
		db_pool
			.first_time_setup()
			.await
			.inspect_err(|e| error!("Error creating tables: {e}"))?;

		info!("Data tables created");
	}

	Ok(db_pool)
}
