use dashmap::DashMap;

use fxhash::FxBuildHasher as BuildHasher;

use log::debug;

use serenity::async_trait;
use serenity::futures::channel::mpsc;
use serenity::model::id::{ChannelId, GuildId, UserId};
use serenity::prelude::{RwLock, TypeMapKey};

use serde::Deserialize;

use songbird::error::TrackError;
use songbird::tracks::TrackHandle;

use uuid::Uuid;

use std::sync::Arc;

use crate::spotify::SpotifyApi;

#[derive(Deserialize)]
pub struct Keys {
	pub database_connect_string: String,
	pub application_id: u64,
	pub token: String,
	pub youtube_api: Option<String>,
	pub spotify_api: Option<SpotifyApi>,
}

impl TypeMapKey for Keys {
	type Value = ArcRw<Keys>;
}

pub struct VoiceUserCache;

pub type ArcRw<T> = Arc<RwLock<T>>;

impl TypeMapKey for VoiceUserCache {
	type Value =
		Arc<DashMap<GuildId, Arc<DashMap<UserId, Option<ChannelId>, BuildHasher>>, BuildHasher>>;
}

pub struct VoiceGuild {
	audios: Vec<TrackHandle>,
	to_remove: mpsc::UnboundedReceiver<Uuid>,
	to_remove_sender: mpsc::UnboundedSender<Uuid>,
}

impl Default for VoiceGuild {
	fn default() -> Self {
		let (to_remove_sender, to_remove) = mpsc::unbounded();
		VoiceGuild {
			audios: Vec::default(),
			to_remove,
			to_remove_sender,
		}
	}
}

impl VoiceGuild {
	pub fn add_audio(
		&mut self,
		audio: TrackHandle,
		volume: f32,
	) -> songbird::error::TrackResult<()> {
		self.clean_audios();

		audio.set_volume(volume)?;
		audio
			.add_event(
				songbird::Event::Track(songbird::TrackEvent::End),
				TrackEventHandler(self.to_remove_sender.clone()),
			)
			.unwrap();
		self.audios.push(audio);
		Ok(())
	}

	pub fn stop(&mut self) {
		for audio in &self.audios {
			let _ = audio.stop();
		}
	}

	pub fn set_volume(&mut self, volume: f32) -> songbird::error::TrackResult<()> {
		for audio in &self.audios {
			match audio.set_volume(volume) {
				Ok(_) | Err(TrackError::Finished) => (),
				Err(e) => return Err(e),
			}
		}
		Ok(())
	}

	fn clean_audios(&mut self) {
		debug!("Cleaning audios. List before cleaning: {:?}", self.audios);

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

		debug!("Cleaned audios. List after cleaning: {:?}", self.audios);
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
	type Value = Arc<DashMap<GuildId, ArcRw<VoiceGuild>, BuildHasher>>;
}
