use serenity::async_trait;
use serenity::builder::CreateApplicationCommand;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::model::interactions::application_command::{
	ApplicationCommand, ApplicationCommandInteraction,
};
use serenity::model::prelude::Interaction;
use serenity::model::voice::VoiceState;
use serenity::prelude::Context;
use serenity::prelude::EventHandler;

use songbird::SongbirdKey;

use crate::configuration::Config;
use crate::data::{VoiceGuilds, VoiceUserCache};
use crate::herald::{
	intro_interaction_create, intro_outro_interaction, introbot_interaction,
	introbot_interaction_create, outro_interaction_create, IntroOutroMode,
};
use crate::util::*;
use crate::voice::PlaySource;
use crate::voice::{
	audio_source, banish_interaction, banish_interaction_create, clip_interaction,
	clip_interaction_create, list_interaction, list_interaction_create, play_interaction,
	play_interaction_create, skip_interaction, skip_interaction_create, stop_interaction,
	stop_interaction_create, summon_interaction, summon_interaction_create, volume_interaction,
	volume_interaction_create,
};
use crate::OPT;

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

		if OPT.reregister {
			println!("Reregistering slash commands...");

			let commands = ctx.http.get_global_application_commands().await.unwrap();

			for command in commands {
				ctx.http
					.delete_global_application_command(command.id.into())
					.await
					.unwrap();
			}
			println!("Deleted old slash commands");

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
					.create_application_command(volume_interaction_create)
					.create_application_command(stop_interaction_create)
					.create_application_command(skip_interaction_create)
					.create_application_command(help_interaction_create)
			})
			.await
			.unwrap();

			if OPT.verbose {
				println!(
					"Registered slash commands: {:#?}",
					ApplicationCommand::get_global_application_commands(&ctx.http).await,
				);
			} else {
				println!("Reregistered slash commands");
			}
		}
	}

	async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
		if let Interaction::ApplicationCommand(command) = interaction {
			println!("Staring interaction name: {:?}, id: {:?}, token: {:?}", command.data.name, command.id, command.token);

			match match command.data.name.as_str() {
				"intro" => intro_outro_interaction(&ctx, &command, IntroOutroMode::Intro).await,
				"outro" => intro_outro_interaction(&ctx, &command, IntroOutroMode::Outro).await,
				"introbot" => introbot_interaction(&ctx, &command).await,
				"summon" => summon_interaction(&ctx, &command).await,
				"banish" => banish_interaction(&ctx, &command).await,
				"list" => list_interaction(&ctx, &command).await,
				"clip" => clip_interaction(&ctx, &command).await,
				"play" => play_interaction(&ctx, &command).await,
				"volume" => volume_interaction(&ctx, &command).await,
				"stop" => stop_interaction(&ctx, &command).await,
				"skip" => skip_interaction(&ctx, &command).await,
				"help" => help_interaction(&ctx, &command).await,
				_ => command.respond_str(&ctx, "Unknown command").await,
			} {
				Ok(_) => (),
				Err(e) => eprintln!("Error running interaction: {:?}", e),
			}

			println!("Completed interaction name: {:?}, id: {:?}, token: {:?}", command.data.name, command.id, command.token);
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
					let config_arc = ctx.data.read().await.clone_expect::<Config>();

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

				let (songbird, voice_guild_arc, volume) = {
					let lock = ctx.data.read().await;

					let songbird = lock.clone_expect::<SongbirdKey>();

					let voice_guild_arc = lock
						.clone_expect::<VoiceGuilds>()
						.write()
						.await
						.entry(guild_id)
						.or_default()
						.clone();

					let volume = lock
						.clone_expect::<Config>()
						.read()
						.await
						.guilds
						.get(&guild_id)
						.and_then(|c| c.volume)
						.unwrap_or(0.5);

					(songbird, voice_guild_arc, volume)
				};

				let mut voice_guild = voice_guild_arc.write().await;

				if let Some(call) = songbird.get(guild_id) {
					let source = audio_source(&clip, PlaySource::Clip).await;

					match source {
						Ok(source) => {
							voice_guild
								.add_audio(call.lock().await.play_source(source), volume)
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

fn help_interaction_create(
	command: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
	command
		.name("help")
		.description("Get help on the commands the bot supports")
}

async fn help_interaction(
	ctx: &Context,
	interaction: &ApplicationCommandInteraction,
) -> serenity::Result<()> {
	interaction
		.respond_str(
			&ctx,
			"Please use !help for now, instead of the slash command",
		)
		.await
}
