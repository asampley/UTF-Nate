use serenity::async_trait;
use serenity::futures::channel::mpsc;
use serenity::model::id::{ChannelId, GuildId, UserId};
use serenity::prelude::{RwLock, TypeMapKey};

use songbird::tracks::TrackHandle;

use uuid::Uuid;

use crate::configuration::Config;

use std::collections::HashMap;
use std::sync::Arc;

pub struct VoiceUserCache;

type ArcRw<T> = Arc<RwLock<T>>;

impl TypeMapKey for VoiceUserCache {
	type Value = ArcRw<HashMap<GuildId, ArcRw<HashMap<UserId, Option<ChannelId>>>>>;
}

pub struct VoiceGuild {
	volume: f32,
	audios: Vec<TrackHandle>,
	to_remove: mpsc::UnboundedReceiver<Uuid>,
	to_remove_sender: mpsc::UnboundedSender<Uuid>,
}

impl Default for VoiceGuild {
	fn default() -> Self {
		let (to_remove_sender, to_remove) = mpsc::unbounded();
		VoiceGuild {
			volume: 0.5,
			audios: Vec::default(),
			to_remove,
			to_remove_sender,
		}
	}
}

pub struct ConfigResource;

impl TypeMapKey for ConfigResource {
	type Value = ArcRw<Config>;
}

impl VoiceGuild {
	pub fn add_audio(&mut self, audio: TrackHandle) -> songbird::error::TrackResult<()> {
		self.clean_audios();

		audio.set_volume(self.volume)?;
		audio
			.add_event(
				songbird::Event::Track(songbird::TrackEvent::End),
				TrackEventHandler(self.to_remove_sender.clone()),
			)
			.unwrap();
		self.audios.push(audio);
		Ok(())
	}

	pub fn set_volume(&mut self, volume: f32) -> songbird::error::TrackResult<()> {
		self.volume = volume;
		for audio in &self.audios {
			audio.set_volume(volume)?;
		}
		Ok(())
	}

	pub fn volume(&self) -> f32 {
		self.volume
	}

	pub fn clear_audios(&mut self) -> songbird::error::TrackResult<()> {
		for audio in self.audios.drain(..) {
			audio.stop()?;
		}
		Ok(())
	}

	fn clean_audios(&mut self) {
		loop {
			match self.to_remove.try_next() {
				Ok(Some(uuid)) => {
					for i in 0..self.audios.len() {
						let audio_uuid = self.audios[i].uuid();

						if audio_uuid == uuid {
							self.audios.swap_remove(i);
							break;
						}
					}
				}
				Ok(None) => unimplemented!(),
				Err(_) => break,
			}
		}
	}
}

pub struct TrackEventHandler(mpsc::UnboundedSender<Uuid>);

#[async_trait]
impl songbird::EventHandler for TrackEventHandler {
	async fn act(&self, ctx: &songbird::EventContext<'_>) -> Option<songbird::Event> {
		match ctx {
			songbird::EventContext::Track(track_events) => {
				for (state, handle) in track_events.iter() {
					if state.playing.is_done() {
						self.0.unbounded_send(handle.uuid()).unwrap();
					}
				}
			}
			_ => (),
		}

		None
	}
}

pub struct VoiceGuilds;

impl TypeMapKey for VoiceGuilds {
	type Value = ArcRw<HashMap<GuildId, ArcRw<VoiceGuild>>>;
}
