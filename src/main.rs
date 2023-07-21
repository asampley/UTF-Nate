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
mod spotify;
mod util;
mod youtube;

use clap::Parser;

use ring::aead::LessSafeKey;

use thiserror::Error;

use tokio::task::JoinSet;

use tracing::{error, info};
use tracing_subscriber::filter::LevelFilter;

use once_cell::sync::Lazy;

use serenity::http::client::Http;
use serenity::model::gateway::GatewayIntents;
use serenity::model::Permissions;
use serenity::prelude::RwLock;

use songbird::serenity::SerenityInit;

use configuration::Config;
use data::{Keys, VoiceGuilds, VoiceUserCache};
use handler::Handler;
use interaction::reregister;
use sqlx::{Executor, PgPool};
use util::{check_msg, read_toml, Context, Framework, FrameworkError, Respond};

use std::fmt::{Debug, Write};
use std::path::Path;
use std::sync::Arc;

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

/// Key for [`serenity::prelude::TypeMap`] to enter the database pool.
struct Pool;

impl serenity::prelude::TypeMapKey for Pool {
	type Value = PgPool;
}

/// Key for [`ring::aead::LessSafeKey`] for encryption purposes.
struct AeadKey;

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
			let mut join_set = JoinSet::<Result<(), ProcessError>>::new();

			info!("Config: {:#?}", *CONFIG);

			// create a framework to process message commands
			let framework = Framework::builder()
				.token(&keys.discord.token)
				.intents(GATEWAY_INTENTS)
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
					pre_command: |ctx| Box::pin(before_hook(ctx)),
					post_command: |ctx| Box::pin(after_hook(ctx)),
					on_error: |err| Box::pin(on_error(err)),
					..Default::default()
				})
				.client_settings(|client_builder| {
					#[cfg(feature = "http-interface")]
					let client_builder = client_builder.type_map_insert::<AeadKey>(encrypt::gen_key());

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
				.build()
				.await
				.expect("Error starting bot");

			#[cfg(feature = "http-interface")]
			if let Some(addr) = &CONFIG.http {
				use axum::routing::*;

				use crate::commands::http::form_endpoint;
				use crate::commands::http::FormRouter;
				use crate::commands::*;

				let client = framework.client();
				let state = commands::BotState {
					data: client.data.clone(),
					cache: client.cache_and_http.cache.clone(),
				};

				info!("Starting HTTP server");

				let app = axum::Router::new()
					.route(
						"/css/index.css",
						get(|| async { include_str!("../web/css/index.css") }),
					)
					.route(
						"/img/loading.svg",
						get(|| async {
							(
								[(axum::http::header::CONTENT_TYPE, "image/svg+xml")],
								include_str!("../web/img/loading.svg"),
							)
						}),
					)
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
					.with_state(state);

				let http_future = hyper::Server::bind(addr).serve(app.into_make_service());

				join_set.spawn(async move { http_future.await.map_err(Into::into) });
			}

			join_set.spawn(async move { framework.start().await.map_err(Into::into) });

			while let Some(res) = join_set.join_next().await {
				match res {
					Ok(res) => res.expect("Failure in joined process"),
					Err(e) => error!("Failed to join: {}", e),
				}
			}
		}
	}
}

/// Load the configuration from `config.toml`.
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

/// Log every execution of a command, before it is executed.
///
/// Information is logged with [`info!()`].
///
/// See [`poise::FrameworkOptions::pre_command`] for more information.
async fn before_hook(ctx: Context<'_>) {
	let guild_name = ctx.guild_id().map(|gid| {
		ctx.serenity_context()
			.cache
			.guild_field(gid, |g| g.name.clone())
	});

	info!(
		"User {} ({}) in guild {:?} ({:?}) running {}",
		ctx.author().name,
		ctx.author().id,
		guild_name,
		ctx.guild_id(),
		ctx.invoked_command_name()
	);
}

/// Log every execution of a command, after it is executed.
///
/// Information is logged with [`info!()`].
///
/// See [`poise::FrameworkOptions::post_command`] for more information.
async fn after_hook(ctx: Context<'_>) {
	let guild_name = ctx.guild_id().map(|gid| {
		ctx.serenity_context()
			.cache
			.guild_field(gid, |g| g.name.clone())
	});

	info!(
		"User {} ({}) in guild {:?} ({:?}) completed {}",
		ctx.author().name,
		ctx.author().id,
		guild_name,
		ctx.guild_id(),
		ctx.invoked_command_name()
	);
}

/// Respond to the command, and depending on the nature of the error, log it.
///
/// Some errors are an issue only with the usage of the command, and should just
/// have a response. Other errors which are an issue with the bot should be
/// logged.
///
/// See [`poise::FrameworkOptions::on_error`] for more information.
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
		ArgumentParse { error, ctx, input } => {
			let mut response = match input {
				Some(input) => format!("Could not parse {:?}. ", input),
				None => String::new(),
			};

			if error.is::<poise::TooManyArguments>() {
				write!(response, "Too many arguments supplied.").unwrap()
			} else if error.is::<poise::TooFewArguments>() {
				write!(response, "Too few arguments supplied.").unwrap()
			} else if error.is::<core::num::ParseFloatError>() {
				write!(response, "Expected a float like 0.25.").unwrap()
			} else if error.is::<commands::queue::ParseLoopArgError>()
				|| error.is::<parser::ParseSelectionError>()
			{
				let mut msg = format!("{}.", error);
				msg[..1].make_ascii_uppercase();
				write!(response, "{}", msg).unwrap()
			} else {
				error!("Unhandled argument parse error: {:?}", error);

				write!(response, "Could not parse arguments for command.").unwrap()
			}

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
