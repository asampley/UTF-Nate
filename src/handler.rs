use serenity::prelude::EventHandler;
use serenity::prelude::Context;
use serenity::model::user::CurrentUser;
use serenity::model::voice::VoiceState;
use serenity::model::gateway::Ready;
use serenity::model::id::{UserId, GuildId, ChannelId};
use serenity::prelude::*;
use serenity::client::CACHE;

use crate::voice::audio_source;
use crate::voice::VoiceManager;

use std::collections::HashMap;

pub struct VoiceCache;

impl TypeMapKey for VoiceCache {
    type Value = HashMap<GuildId, HashMap<UserId, Option<ChannelId>>>;
}

pub struct Handler;

// implement default event handler
impl EventHandler for Handler {
    fn ready(&self, _: Context, _: Ready) {
        println!("Bot started!");

        println!("Bot info {:?}", serenity::CACHE.read().user);
    }

    fn voice_state_update(&self, ctx: Context, guild_id: Option<GuildId>, voice_state: VoiceState) {
        if let Some(guild_id) = guild_id {
            let mut data_lock = ctx.data.lock();
            let cache = data_lock
                .get_mut::<VoiceCache>()
                .expect("Expected VoiceCache in ShareMap");

            let bot_channel = *cache
                .entry(guild_id).or_default()
                .entry(bot_id()).or_insert(None);
            let user_channel = cache
                .entry(guild_id).or_default()
                .entry(voice_state.user_id).or_insert(None);

            let previous_channel = *user_channel;

            println!("{:?} moved from {:?} to {:?}", voice_state.user_id, user_channel, voice_state.channel_id);

            *user_channel = voice_state.channel_id;

            let intro = "dota/helloits";
            let outro = "bnw/death";

            if bot_channel != None && voice_state.user_id != bot_id() {
                let clip = if *user_channel == bot_channel {
                    intro
                } else if previous_channel == bot_channel {
                    outro
                } else {
                    return;
                };

                let manager_lock = data_lock.get::<VoiceManager>().cloned().expect("Expected VoiceManager in ShareMap");
                let mut manager = manager_lock.lock();

                if let Some(handler) = manager.get_mut(guild_id) {
                    let source = audio_source(clip);

                    match source {
                        Ok(source) => handler.play(source),
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
