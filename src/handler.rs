use log::{debug, error, info};

use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::model::interactions::application_command::ApplicationCommand;
use serenity::model::prelude::{Activity, Interaction};
use serenity::model::voice::VoiceState;
use serenity::prelude::Context;
use serenity::prelude::EventHandler as SerenityEventHandler;

use songbird::SongbirdKey;

use crate::Pool;

use crate::audio::clip_source;
use crate::commands::help::{help_interaction, help_interaction_create};
use crate::commands::herald::{
	intro_interaction_create, intro_outro_interaction, introbot_interaction,
	introbot_interaction_create, outro_interaction_create, IntroOutroMode,
};
use crate::commands::join::{
	banish_interaction, banish_interaction_create, summon_interaction, summon_interaction_create,
};
use crate::commands::play::{
	clip_interaction, clip_interaction_create, play_interaction, play_interaction_create,
	playnext_interaction, playnext_interaction_create, playnow_interaction,
	playnow_interaction_create,
};
use crate::commands::queue::{
	pause_interaction, pause_interaction_create, queue_interaction, queue_interaction_create,
	shuffle_interaction, shuffle_interaction_create, shufflenow_interaction,
	shufflenow_interaction_create, skip_interaction, skip_interaction_create, stop_interaction,
	stop_interaction_create, unpause_interaction, unpause_interaction_create,
};
use crate::commands::voice::{
	list_interaction, list_interaction_create, volume_interaction, volume_interaction_create,
};
use crate::configuration::Config;
use crate::data::{VoiceGuilds, VoiceUserCache};
use crate::util::*;
use crate::OPT;

pub struct Handler;

enum IOClip {
	Intro,
	Outro,
}

#[async_trait]
impl SerenityEventHandler for Handler {
	async fn ready(&self, ctx: Context, _: Ready) {
		info!("Bot started!");

		info!("Bot info {:?}", ctx.cache.current_user_id().await);

		ctx.set_activity(Activity::watching("you. /help")).await;

		if OPT.reregister {
			info!("Reregistering slash commands...");

			let commands = ctx.http.get_global_application_commands().await.unwrap();

			for command in commands {
				ctx.http
					.delete_global_application_command(command.id.into())
					.await
					.unwrap();
			}
			info!("Deleted old slash commands");

			ApplicationCommand::set_global_application_commands(&ctx.http, |commands| {
				commands
					.create_application_command(intro_interaction_create)
					.create_application_command(outro_interaction_create)
					.create_application_command(introbot_interaction_create)
					.create_application_command(summon_interaction_create)
					.create_application_command(banish_interaction_create)
					.create_application_command(list_interaction_create)
					.create_application_command(clip_interaction_create)
					.create_application_command(play_interaction_create)
					.create_application_command(playnext_interaction_create)
					.create_application_command(playnow_interaction_create)
					.create_application_command(volume_interaction_create)
					.create_application_command(stop_interaction_create)
					.create_application_command(skip_interaction_create)
					.create_application_command(pause_interaction_create)
					.create_application_command(unpause_interaction_create)
					.create_application_command(queue_interaction_create)
					.create_application_command(shuffle_interaction_create)
					.create_application_command(shufflenow_interaction_create)
					.create_application_command(help_interaction_create)
			})
			.await
			.unwrap();

			debug!(
				"Registered slash commands: {:#?}",
				ApplicationCommand::get_global_application_commands(&ctx.http).await,
			);
			info!("Reregistered slash commands");
		}
	}

	async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
		if let Interaction::ApplicationCommand(command) = interaction {
			info!(
				"Staring interaction name: {:?}, id: {:?}, token: {:?}",
				command.data.name, command.id, command.token
			);

			match match command.data.name.as_str() {
				"intro" => intro_outro_interaction(&ctx, &command, IntroOutroMode::Intro).await,
				"outro" => intro_outro_interaction(&ctx, &command, IntroOutroMode::Outro).await,
				"introbot" => introbot_interaction(&ctx, &command).await,
				"summon" => summon_interaction(&ctx, &command).await,
				"banish" => banish_interaction(&ctx, &command).await,
				"list" => list_interaction(&ctx, &command).await,
				"clip" => clip_interaction(&ctx, &command).await,
				"play" => play_interaction(&ctx, &command).await,
				"playnext" => playnext_interaction(&ctx, &command).await,
				"playnow" => playnow_interaction(&ctx, &command).await,
				"volume" => volume_interaction(&ctx, &command).await,
				"stop" => stop_interaction(&ctx, &command).await,
				"skip" => skip_interaction(&ctx, &command).await,
				"pause" => pause_interaction(&ctx, &command).await,
				"unpause" => unpause_interaction(&ctx, &command).await,
				"queue" => queue_interaction(&ctx, &command).await,
				"shuffle" => shuffle_interaction(&ctx, &command).await,
				"shufflenow" => shufflenow_interaction(&ctx, &command).await,
				"help" => help_interaction(&ctx, &command).await,
				_ => command.respond_err(&ctx, &"Unknown command".into()).await,
			} {
				Ok(_) => (),
				Err(e) => error!("Error running interaction: {:?}", e),
			}

			info!(
				"Completed interaction name: {:?}, id: {:?}, token: {:?}",
				command.data.name, command.id, command.token
			);
		}
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
					.entry(guild_id)
					.or_default()
					.clone();

				let bot_id = ctx.cache.current_user_id().await;

				// update cache if the user is the bot
				if new_state.user_id == bot_id {
					cache_guild.insert(bot_id, new_state.channel_id);
				}

				// get the bot's channel
				let bot_channel = cache_guild
					.get(&bot_id)
					.map(|r| r.value().clone())
					.flatten();

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
					let config_arc = ctx.data.read().await.clone_expect::<Config>();

					let pool = ctx.data.read().await.clone_expect::<Pool>();

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
							IOClip::Intro =>
								Config::get_intro(&pool, &new_state.user_id)
									.await
									.map_err(|e| error!("Error fetching intro: {:?}", e))
									.ok()
									.flatten()
									.unwrap_or("bnw/cowhappy".to_owned()),
							IOClip::Outro =>
								Config::get_outro(&pool, &new_state.user_id)
									.await
									.map_err(|e| error!("Error fetching outro: {:?}", e))
									.ok()
									.flatten()
									.unwrap_or("bnw/death".to_owned()),
						}
					}
				};

				let (songbird, voice_guild_arc, volume) = {
					let lock = ctx.data.read().await;

					let songbird = lock.clone_expect::<SongbirdKey>();

					let voice_guild_arc = lock
						.clone_expect::<VoiceGuilds>()
						.entry(guild_id)
						.or_default()
						.clone();

					let volume = lock
						.clone_expect::<Config>()
						.read()
						.await
						.guilds
						.get(&guild_id)
						.and_then(|c| c.volume_clip)
						.unwrap_or(0.5);

					(songbird, voice_guild_arc, volume)
				};

				let mut voice_guild = voice_guild_arc.write().await;

				if let Some(call) = songbird.get(guild_id) {
					match clip_source(&clip).await {
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
								"Error trying to play {} clip: {:?}",
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
