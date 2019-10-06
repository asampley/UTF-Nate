use serenity::prelude::EventHandler;
use serenity::prelude::Context;
use serenity::model::voice::VoiceState;
use serenity::model::gateway::Ready;
use serenity::model::id::{UserId, GuildId};
use serenity::client::CACHE;

use crate::voice::audio_source;

use crate::data::{VoiceGuilds, VoiceUserCache, VoiceManager, ConfigResource};

pub struct Handler;

enum IOClip { Intro, Outro }

// implement default event handler
impl EventHandler for Handler {
    fn ready(&self, _: Context, _: Ready) {
        println!("Bot started!");

        println!("Bot info {:?}", serenity::CACHE.read().user);
    }

    fn voice_state_update(&self, ctx: Context, guild_id: Option<GuildId>, voice_state: VoiceState) {
        if let Some(guild_id) = guild_id {
            let cache_guild = ctx.data.lock()
                .get::<VoiceUserCache>()
                .cloned()
                .expect("Expected VoiceUserCache in ShareMap")
                .write()
                .entry(guild_id).or_default()
                .clone();

            let (bot_channel, previous_channel, user_channel) = {
                let mut cache_lock = cache_guild.write();

                // get previous channel for the user
                let user_channel = cache_lock
                    .entry(voice_state.user_id)
                    .or_insert(None);

                // store previous channel and update
                let previous_channel = *user_channel;
                *user_channel = voice_state.channel_id;
                let user_channel = *user_channel;

                // get the bot's channel, which may be updated at this point.
                let bot_channel = *cache_lock
                    .entry(bot_id())
                    .or_insert(None);

                //let bot_channel = if voice_state.user_id == bot_id() {
                    //user_channel
                //} else {
                    //bot_channel
                //};

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
                    let config_arc = ctx.data.lock()
                        .get::<ConfigResource>()
                        .cloned()
                        .expect("Expected ConfigResource in ShareMap")
                        .clone();

                    let config = config_arc.read();

                    if voice_state.user_id == bot_id() {
                        match io {
                            IOClip::Intro => config.guilds
                                .get(&guild_id)
                                .and_then(|gc| gc.bot_intro.as_ref())
                                .map(|s| s.as_str())
                                .unwrap_or("dota/bothello")
                                .to_owned(),
                            IOClip::Outro => return,
                        }
                    } else {
                        match io {
                            IOClip::Intro => config.intros
                                .get(&voice_state.user_id).map(|s| s.as_str())
                                .unwrap_or("bnw/cowhappy")
                                .to_owned(),
                            IOClip::Outro => config.outros
                                .get(&voice_state.user_id).map(|s| s.as_str())
                                .unwrap_or("bnw/death")
                                .to_owned(),
                        }
                    }
                };

                let manager_lock = ctx.data.lock().get::<VoiceManager>().cloned().expect("Expected VoiceManager in ShareMap");
                let mut manager = manager_lock.lock();

                let voice_guild_arc = ctx.data.lock().get_mut::<VoiceGuilds>().cloned()
                    .expect("Expected VoiceGuilds in ShareMap")
                    .write()
                    .entry(guild_id)
                    .or_default()
                    .clone();

                let mut voice_guild = voice_guild_arc.write();

                if let Some(handler) = manager.get_mut(guild_id) {
                    let source = audio_source(&clip);

                    match source {
                        Ok(source) => {
                            voice_guild.add_audio(handler.play_returning(source));
                            println!("Playing {} for user ({})",
                                match io { IOClip::Intro => "intro", IOClip::Outro => "outro" },
                                voice_state.user_id
                            );
                        },
                        Err(reason) => {
                            eprintln!("Error trying to intro clip: {:?}", reason);
                        }
                    }
                }
            }
        }
    }
}

fn bot_id() -> UserId {
    CACHE.read().user.id
}
