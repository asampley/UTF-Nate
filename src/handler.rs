use tracing::{error, info};

use serenity::async_trait;
use serenity::client::Context as SerenityContext;
use serenity::model::gateway::Ready;
use serenity::model::prelude::Activity;
use serenity::model::voice::VoiceState;
use serenity::prelude::EventHandler as SerenityEventHandler;

use songbird::SongbirdKey;

use crate::Pool;

use crate::audio::clip_source;
use crate::configuration::Config;
use crate::data::{VoiceGuilds, VoiceUserCache};
use crate::util::*;

pub struct Handler;

enum IOClip {
	Intro,
	Outro,
}

#[async_trait]
impl SerenityEventHandler for Handler {
	async fn ready(&self, ctx: SerenityContext, _: Ready) {
		info!("Bot started!");

		info!("Bot info {:?}", ctx.cache.current_user_id());

		ctx.set_activity(Activity::watching("you.")).await;
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

			if bot_channel != None {
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
								.unwrap_or_else(|| "bnw/cow happy".to_owned()),
							IOClip::Outro => Config::get_outro(&pool, &new_state.user_id)
								.await
								.map_err(|e| error!("Error fetching outro: {:?}", e))
								.ok()
								.flatten()
								.unwrap_or_else(|| "bnw/death".to_owned()),
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
