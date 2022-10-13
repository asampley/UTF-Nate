//! Definitions of data like API keys and runtime data for guild voice state.

use dashmap::DashMap;

use fxhash::FxBuildHasher as BuildHasher;

use tracing::debug;

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
use crate::youtube::YoutubeApi;

/// Authentication keys required to connect to assorted external interfaces.
#[derive(Deserialize)]
pub struct Keys {
	/// Connection info for the database.
	pub database: DatabaseKeys,

	/// Connection info for the discord API.
	pub discord: DiscordApi,

	/// Connection info for the youtube API, if set up.
	pub youtube: Option<YoutubeApi>,

	/// Connection info for the spotify API, if set up.
	pub spotify: Option<SpotifyApi>,
}

/// Token and application id for connecting to the discord API.
#[derive(Deserialize)]
pub struct DiscordApi {
	pub application_id: u64,
	pub token: String,
}

/// Connection string for the database, which includes credentials if
/// necessary.
#[derive(Deserialize)]
pub struct DatabaseKeys {
	pub connect_string: String,
}

/// Allow these keys to be inserted into a typemap.
impl TypeMapKey for Keys {
	type Value = ArcRw<Keys>;
}

/// Empty struct to be a [`TypeMapKey`].
pub struct VoiceUserCache;

/// Alias for `Arc<RwLock<T>>`.
pub type ArcRw<T> = Arc<RwLock<T>>;

/// Allow a cache of users mapped to channels, for tracking when to intro and
/// outro, for example.
impl TypeMapKey for VoiceUserCache {
	type Value =
		Arc<DashMap<GuildId, Arc<DashMap<UserId, Option<ChannelId>, BuildHasher>>, BuildHasher>>;
}

/// Collection of audios that have been queued.
pub struct VoiceGuild {
	audios: Vec<TrackHandle>,
	to_remove: mpsc::UnboundedReceiver<Uuid>,
	to_remove_sender: mpsc::UnboundedSender<Uuid>,
}

impl Default for VoiceGuild {
	/// Create an empty list of audios.
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
	/// Add an audio with the specified volume.
	///
	/// Before the audio is added, any audios that need to be cleaned up are
	/// first cleared with [`Self::clean_audios`].
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

	/// Stop all audios. See [`TrackHandle::stop`].
	pub fn stop(&mut self) {
		for audio in &self.audios {
			let _ = audio.stop();
		}
	}

	/// Set the volume for all audios contained within.
	pub fn set_volume(&mut self, volume: f32) -> songbird::error::TrackResult<()> {
		for audio in &self.audios {
			match audio.set_volume(volume) {
				Ok(_) | Err(TrackError::Finished) => (),
				Err(e) => return Err(e),
			}
		}
		Ok(())
	}

	/// Clean up audios that have been added to be removed by the sender side
	/// of the channel.
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

/// Send events when the tracks finish, to remove them from a [`VoiceGuild`],
/// for example.
pub struct TrackEventHandler(mpsc::UnboundedSender<Uuid>);

#[async_trait]
impl songbird::EventHandler for TrackEventHandler {
	async fn act(&self, ctx: &songbird::EventContext<'_>) -> Option<songbird::Event> {
		if let songbird::EventContext::Track(track_events) = ctx {
			for (state, handle) in track_events.iter() {
				if state.playing.is_done() {
					self.0.unbounded_send(handle.uuid()).unwrap();
				}
			}
		};

		None
	}
}

/// Allow storing a [`VoiceGuild`] for each guild.
pub struct VoiceGuilds;

impl TypeMapKey for VoiceGuilds {
	type Value = Arc<DashMap<GuildId, ArcRw<VoiceGuild>, BuildHasher>>;
}
