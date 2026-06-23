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

use thiserror::Error;

use tokio::task::JoinSet;

use tracing::{error, info};
use tracing_subscriber::filter::LevelFilter;

use serenity::client::Client;
use serenity::http::Http;
use serenity::model::Permissions;
use serenity::model::gateway::GatewayIntents;
use serenity::prelude::RwLock;

use songbird::serenity::SerenityInit;

use configuration::Config;
use data::{Keys, VoiceGuilds, VoiceUserCache};
use handler::Handler;
use interaction::reregister;
use util::{Framework, read_toml};

use std::fmt::Debug;
use std::path::Path;
use std::sync::{Arc, LazyLock};

use crate::persistence::Storage;

/// Path to shared resources directory for things such as clips or database scripts.
static RESOURCE_PATH: LazyLock<&'static Path> = LazyLock::new(|| Path::new("resources/"));

/// Options parsed from the command line using [`clap`].
static OPT: LazyLock<Opt> = LazyLock::new(Opt::parse);

/// Configuration parameters from a file. See [`load_config()`].
static CONFIG: LazyLock<Config> = LazyLock::new(load_config);

static REQWEST_CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| {
	let builder = reqwest::ClientBuilder::new();

	#[cfg(feature = "tls-rustls")]
	let builder = builder.use_rustls_tls();
	#[cfg(feature = "tls-native-tls")]
	let builder = builder.use_native_tls();

	builder
		.build()
		.expect("Unable to initialize reqwest client")
});

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
	type Value = Box<dyn Storage + Send + Sync>;
}

/// Key for [`ring::aead::LessSafeKey`] for encryption purposes.
#[allow(dead_code)]
enum AeadKey {}

impl serenity::prelude::TypeMapKey for AeadKey {
	type Value = LessSafeKey;
}

#[derive(Debug, Parser)]
struct Opt {
	/// Generate a key for tokens. Place it in keys.toml. This stops the command immediately after
	/// printing the key.
	#[cfg(feature = "http-interface")]
	#[arg(long)]
	generate_key: bool,

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
	#[arg(long, short, action = clap::ArgAction::Count)]
	verbose: u8,

	/// Do not check for clip collisions. Speeds up start by disabling.
	#[arg(long)]
	check_clips: bool,
}

#[derive(Debug, Error)]
enum ProcessError {
	#[error(transparent)]
	Serenity(#[from] serenity::prelude::SerenityError),

	#[cfg(feature = "http-interface")]
	#[error(transparent)]
	StdIo(#[from] std::io::Error),
}

#[tokio::main]
async fn main() {
	// initialize logging
	let subscriber = tracing_subscriber::fmt()
		.with_max_level(match OPT.verbose {
			0 => LevelFilter::INFO,
			1 => LevelFilter::DEBUG,
			2.. => LevelFilter::TRACE,
		})
		.compact()
		.finish();

	if let Err(e) = tracing::subscriber::set_global_default(subscriber) {
		error!("Unable to set default tracing subscriber: {:?}", e);
		return;
	}

	#[cfg(feature = "http-interface")]
	if OPT.generate_key {
		println!("{}", encrypt::gen_key());

		return;
	}

	println!("Options: {:#?}", OPT);

	if OPT.check_clips {
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

	let commands = &*commands::COMMANDS;

	if OPT.reregister {
		match reregister(&http, commands).await {
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
			#[cfg(feature = "http-interface")]
			let encrypt_key = {
				let chars = keys.encrypt.hex.chars().collect::<Vec<_>>();

				encrypt::key_from_hex(chars[..].try_into().unwrap_or_else(|_| {
					panic!(
						"Wrong key size. Expected a 64 character hex key, got {}",
						chars.len()
					)
				}))
			};

			let mut join_set = JoinSet::<Result<(), ProcessError>>::new();

			info!("Config: {:#?}", *CONFIG);

			// create a framework to process message commands
			let client_builder = Client::builder(&keys.discord.token, GATEWAY_INTENTS)
				.event_handler(Handler::default())
				.type_map_insert::<VoiceUserCache>(Default::default())
				.type_map_insert::<VoiceGuilds>(Default::default())
				.type_map_insert::<Keys>(Arc::new(RwLock::new(keys)))
				.type_map_insert::<StorageKey>(Box::new(db_pool))
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
							commands: commands::commands(),
							pre_command: |ctx| Box::pin(handler::before_hook(ctx)),
							post_command: |ctx| Box::pin(handler::after_hook(ctx)),
							on_error: |err| Box::pin(handler::on_error(err)),
							..Default::default()
						})
						.build(),
				);

			#[cfg(feature = "http-interface")]
			let client_builder = client_builder.type_map_insert::<AeadKey>(encrypt_key.unwrap());

			let mut client = match client_builder.await {
				Ok(client) => client,
				Err(e) => {
					error!("Error starting bot: {:?}", e);
					return;
				}
			};

			#[cfg(feature = "http-interface")]
			if let Some(http_config) = &CONFIG.http {
				let state = commands::BotState {
					data: client.data.clone(),
					cache: client.cache.clone(),
					http: client.http.clone(),
				};

				join_set.spawn(async move {
					http::axum_task(http_config, state)
						.await
						.map_err(Into::into)
				});
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
		.inspect(|_| info!("Read config file from {:?}", path))
		.inspect_err(|e| error!("{:?}", e))
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
