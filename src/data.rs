use serenity::model::id::{GuildId, UserId, ChannelId};
use serenity::voice::LockedAudio;
use serenity::prelude::{Mutex, RwLock, TypeMapKey};
use serenity::client::bridge::voice::ClientVoiceManager;

use crate::configuration::Config;

use std::collections::HashMap;
use std::sync::Arc;

pub struct VoiceUserCache;

type ArcRw<T> = Arc<RwLock<T>>;

impl TypeMapKey for VoiceUserCache {
    type Value = ArcRw<HashMap<GuildId, ArcRw<HashMap<UserId, Option<ChannelId>>>>>;
}

pub struct VoiceManager;

impl TypeMapKey for VoiceManager {
    type Value = Arc<Mutex<ClientVoiceManager>>;
}

pub struct VoiceGuild {
    volume: f32,
    audios: Vec<LockedAudio>,
}

pub struct ConfigResource;

impl TypeMapKey for ConfigResource {
    type Value = ArcRw<Config>;
}

impl VoiceGuild {
    pub async fn add_audio(&mut self, audio: LockedAudio) {
        self.clean_audios().await;
        audio.lock().await.volume(self.volume);
        self.audios.push(audio);
    }

    pub async fn set_volume(&mut self, volume: f32) {
        self.volume = volume;
        for audio in &self.audios {
            audio.lock().await.volume(volume);
        }
    }

    pub async fn volume(&self) -> f32 {
        self.volume
    }

    pub async fn clear_audios(&mut self) {
        for audio in self.audios.drain(..) {
            audio.lock().await.volume(0.0);
        }
    }

    async fn clean_audios(&mut self) {
        let mut i = 0;
        while i != self.audios.len() {
            if !self.audios[i].lock().await.finished {
                self.audios.remove(i);
            } else {
                i += 1;
            }
        }
    }
}

impl Default for VoiceGuild {
    fn default() -> Self {
        VoiceGuild { volume: 0.5, audios: Vec::default() }
    }
}

pub struct VoiceGuilds;

impl TypeMapKey for VoiceGuilds {
    type Value = ArcRw<HashMap<GuildId, ArcRw<VoiceGuild>>>;
}
