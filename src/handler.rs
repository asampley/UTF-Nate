use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::model::voice::VoiceState;
use serenity::prelude::Context;
use serenity::prelude::EventHandler;

use songbird::SongbirdKey;

use crate::data::{ConfigResource, VoiceGuilds, VoiceUserCache};
use crate::voice::audio_source;
use crate::util::*;

pub struct Handler;

enum IOClip {
	Intro,
	Outro,
}

// implement default event handler
#[async_trait]
impl EventHandler for Handler {
	async fn ready(&self, ctx: Context, _: Ready) {
		println!("Bot started!");

		println!("Bot info {:?}", ctx.cache.current_user_id().await);
	}

	async fn voice_state_update(
		&self,
		ctx: Context,
		guild_id: Option<GuildId>,
		old_state: Option<VoiceState>,
		new_state: VoiceState,
	) {
		if let Some(guild_id) = guild_id {
			let (bot_channel, previous_channel, user_channel) = {
				let cache_guild = ctx
					.data
					.read()
					.await
					.clone_expect::<VoiceUserCache>()
					.write()
					.await
					.entry(guild_id)
					.or_default()
					.clone();

				let bot_id = ctx.cache.current_user_id().await;

				// update cache if the user is the bot
				if new_state.user_id == bot_id {
					cache_guild
						.write()
						.await
						.insert(bot_id, new_state.channel_id);
				}

				// get the bot's channel
				let bot_channel = cache_guild.read().await.get(&bot_id).cloned().flatten();

				// get previous channel for the user
				let previous_channel = old_state.map(|s| s.channel_id).flatten();
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
					let config_arc = ctx
						.data
						.read()
						.await
						.clone_expect::<ConfigResource>();

					let config = config_arc.read().await;

					if new_state.user_id == ctx.cache.current_user_id().await {
						match io {
							IOClip::Intro => config
								.guilds
								.get(&guild_id)
								.and_then(|gc| gc.bot_intro.as_ref())
								.map(|s| s.as_str())
								.unwrap_or("dota/bothello")
								.to_owned(),
							IOClip::Outro => return,
						}
					} else {
						match io {
							IOClip::Intro => config
								.intros
								.get(&new_state.user_id)
								.map(|s| s.as_str())
								.unwrap_or("bnw/cowhappy")
								.to_owned(),
							IOClip::Outro => config
								.outros
								.get(&new_state.user_id)
								.map(|s| s.as_str())
								.unwrap_or("bnw/death")
								.to_owned(),
						}
					}
				};

				let songbird = ctx
					.data
					.read()
					.await
					.clone_expect::<SongbirdKey>();

				let voice_guild_arc = ctx
					.data
					.write()
					.await
					.clone_expect::<VoiceGuilds>()
					.write()
					.await
					.entry(guild_id)
					.or_default()
					.clone();

				let mut voice_guild = voice_guild_arc.write().await;

				if let Some(call) = songbird.get(guild_id) {
					let source = audio_source(&clip).await;

					match source {
						Ok(source) => {
							voice_guild
								.add_audio(call.lock().await.play_source(source))
								.expect("Error playing source");
							println!(
								"Playing {} for user ({})",
								match io {
									IOClip::Intro => "intro",
									IOClip::Outro => "outro",
								},
								new_state.user_id
							);
						}
						Err(reason) => {
							eprintln!("Error trying to intro clip: {:?}", reason);
						}
					}
				}
			}
		}
	}
}
