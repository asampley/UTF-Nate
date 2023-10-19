//! Handle events other than regular commands.
//!
//! This is how the intro and outro clips are played when voice state changes
//! are detected.

use dashmap::DashMap;
use rand::rngs::StdRng;
use rand::seq::IteratorRandom;
use rand::SeedableRng;
use tracing::{error, info};

use serenity::async_trait;
use serenity::client::Context as SerenityContext;
use serenity::model::gateway::Ready;
use serenity::model::prelude::UserId;
use serenity::model::user::OnlineStatus;
use serenity::model::voice::VoiceState;
use serenity::prelude::EventHandler as SerenityEventHandler;

use songbird::SongbirdKey;

use std::fmt::Write;

use crate::Pool;

use crate::audio::{clip_iter, clip_source};
use crate::configuration::Config;
use crate::data::{VoiceGuilds, VoiceUserCache};
use crate::util::*;

/// Handler that handeles serenity events for playing intros and outros, and
/// other non-command events.
#[derive(Default)]
pub struct Handler {
	random_audio_cache: DashMap<u64, String>,
}

/// Enum tagging either an intro or outro.
enum IOClip {
	Intro,
	Outro,
}

impl Handler {
	fn random_clip(&self, seed: u64) -> String {
		self.random_audio_cache
			.entry(seed)
			.or_insert_with(|| {
				clip_iter()
					.choose(&mut StdRng::seed_from_u64(seed))
					.expect("No clips found")
					.to_string_lossy()
					.into_owned()
			})
			.value()
			.to_owned()
	}

	fn random_intro(&self, user_id: UserId) -> String {
		self.random_clip(user_id.0)
	}

	fn random_outro(&self, user_id: UserId) -> String {
		self.random_clip(!user_id.0)
	}
}

#[async_trait]
impl SerenityEventHandler for Handler {
	async fn ready(&self, ctx: SerenityContext, _: Ready) {
		info!("Bot started!");

		info!("Bot info {:?}", ctx.cache.current_user_id());

		ctx.set_presence(
			crate::CONFIG.activity.as_ref().map(Into::into),
			OnlineStatus::Online,
		)
		.await;
	}

	async fn voice_state_update(
		&self,
		ctx: SerenityContext,
		old_state: Option<VoiceState>,
		new_state: VoiceState,
	) {
		if let Some(guild_id) = new_state.guild_id {
			let (bot_channel, previous_channel, user_channel) = {
				let cache_guild = ctx
					.data
					.read()
					.await
					.clone_expect::<VoiceUserCache>()
					.entry(guild_id)
					.or_default()
					.clone();

				let bot_id = ctx.cache.current_user_id();

				// update cache if the user is the bot
				if new_state.user_id == bot_id {
					cache_guild.insert(bot_id, new_state.channel_id);
				}

				// get the bot's channel
				let bot_channel = cache_guild.get(&bot_id).and_then(|r| *r.value());

				// get previous channel for the user
				let previous_channel = old_state.and_then(|s| s.channel_id);
				let user_channel = new_state.channel_id;

				(bot_channel, previous_channel, user_channel)
			};

			if bot_channel.is_some() {
				let io = if user_channel == previous_channel {
					return;
				} else if user_channel == bot_channel {
					IOClip::Intro
				} else if previous_channel == bot_channel {
					IOClip::Outro
				} else {
					return;
				};

				let clip = {
					let pool = ctx.data.read().await.clone_expect::<Pool>();

					if new_state.user_id == ctx.cache.current_user_id() {
						match io {
							IOClip::Intro => Config::get_bot_intro(&pool, &guild_id)
								.await
								.map_err(|e| error!("Error fetching intro: {:?}", e))
								.ok()
								.flatten()
								.unwrap_or_else(|| "dota/bleep bloop I am a robot".to_owned()),
							IOClip::Outro => return,
						}
					} else {
						match io {
							IOClip::Intro => Config::get_intro(&pool, &new_state.user_id)
								.await
								.map_err(|e| error!("Error fetching intro: {:?}", e))
								.ok()
								.flatten()
								.unwrap_or_else(|| self.random_intro(new_state.user_id)),
							IOClip::Outro => Config::get_outro(&pool, &new_state.user_id)
								.await
								.map_err(|e| error!("Error fetching outro: {:?}", e))
								.ok()
								.flatten()
								.unwrap_or_else(|| self.random_outro(new_state.user_id)),
						}
					}
				};

				let (songbird, voice_guild_arc, volume) = {
					let lock = ctx.data.read().await;

					let songbird = lock.clone_expect::<SongbirdKey>();
					let pool = lock.clone_expect::<Pool>();

					let voice_guild_arc = lock
						.clone_expect::<VoiceGuilds>()
						.entry(guild_id)
						.or_default()
						.clone();

					let volume = Config::get_volume_clip(&pool, &guild_id)
						.await
						.map_err(|e| error!("Unable to get clip volume: {:?}", e))
						.ok()
						.flatten()
						.unwrap_or(0.5);

					(songbird, voice_guild_arc, volume)
				};

				let mut voice_guild = voice_guild_arc.write().await;

				if let Some(call) = songbird.get(guild_id) {
					match clip_source(clip.as_ref()).await {
						Ok(source) => {
							voice_guild
								.add_audio(call.lock().await.play_source(source), volume)
								.expect("Error playing source");
							info!(
								"Playing {} for user ({})",
								match io {
									IOClip::Intro => "intro",
									IOClip::Outro => "outro",
								},
								new_state.user_id
							);
						}
						Err(reason) => {
							error!(
								"Error trying to play {} clip: {}",
								match io {
									IOClip::Intro => "intro",
									IOClip::Outro => "outro",
								},
								reason
							);
						}
					}
				}
			}
		}
	}
}

/// Log every execution of a command, before it is executed.
///
/// Information is logged with [`info!()`].
///
/// See [`poise::FrameworkOptions::pre_command`] for more information.
pub async fn before_hook(ctx: Context<'_>) {
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
pub async fn after_hook(ctx: Context<'_>) {
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
pub async fn on_error(err: FrameworkError<'_>) {
	use poise::FrameworkError as E;

	match &err {
		E::GuildOnly { ctx } | E::DmOnly { ctx } | E::NsfwOnly { ctx } => {
			check_msg(
				ctx.respond_err(
					&format!(
						"`{}{}` is only available in {}",
						ctx.prefix(),
						ctx.command().qualified_name,
						match &err {
							E::GuildOnly { .. } => "guilds",
							E::DmOnly { .. } => "dms",
							E::NsfwOnly { .. } => "nsfw channels",
							_ => unreachable!(),
						}
					)
					.into(),
				)
				.await,
			);
		}
		E::ArgumentParse { error, ctx, input } => {
			let mut response = match input {
				Some(input) => format!("Could not parse {:?}. ", input),
				None => String::new(),
			};

			if error.is::<poise::TooManyArguments>() {
				response.push_str("Too many arguments supplied.")
			} else if error.is::<poise::TooFewArguments>() {
				response.push_str("Too few arguments supplied.")
			} else if error.is::<core::num::ParseFloatError>() {
				response.push_str("Expected a float like 0.25.")
			} else if error.is::<crate::commands::queue::ParseLoopArgError>()
				|| error.is::<crate::parser::ParseSelectionError>()
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
