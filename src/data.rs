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
    pub fn add_audio(&mut self, audio: LockedAudio) {
        self.clean_audios();
        audio.lock().volume(self.volume);
        self.audios.push(audio);
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume;
        self.audios.iter().for_each(|audio| { audio.lock().volume(volume); });
    }

    pub fn volume(&self) -> f32 {
        self.volume
    }

    pub fn clear_audios(&mut self) {
        self.audios.drain(..).for_each(|audio| { audio.lock().volume(0.0); });
    }

    fn clean_audios(&mut self) {
        self.audios.retain(|audio| !audio.lock().finished);
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
