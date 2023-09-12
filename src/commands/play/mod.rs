use tracing::{debug, error};

use serde::{Deserialize, Serialize};

use serenity::prelude::Mutex;

use songbird::create_player;
use songbird::input::Input;
use songbird::Call;
use songbird::SongbirdKey;

use std::sync::Arc;

use crate::audio::{clip_source, play_sources};
use crate::audio::{AudioError, PlayStyle};
use crate::commands::{BotState, Source};
use crate::configuration::Config;
use crate::data::{ArcRw, Keys, VoiceGuild, VoiceGuilds};
use crate::util::write_duration;
use crate::util::{GetExpect, Response};
use crate::Pool;

#[cfg(feature = "http-interface")]
pub mod http;
pub mod poise;

pub const fn clip_help() -> &'static str {
	include_str!("clip.md")
}

pub const fn play_help() -> &'static str {
	include_str!("play.md")
}

pub const fn playnext_help() -> &'static str {
	include_str!("playnext.md")
}

pub const fn playnow_help() -> &'static str {
	include_str!("playnow.md")
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PlayArgs {
	search: String,
}

#[tracing::instrument(level = "info", ret, skip(state))]
pub async fn play(
	state: &BotState,
	source: &Source,
	play_style: PlayStyle,
	play_index: Option<usize>,
	args: &PlayArgs,
) -> Result<Response, Response> {
	let guild_id = source
		.guild_id
		.ok_or("This command is only available in guilds")?;

	let (songbird, voice_guild_arc, volume, keys) = {
		debug!("Acquiring lock for play");

		let data_lock = state.data.read().await;

		debug!("Acquired lock for play");

		let songbird = data_lock.clone_expect::<SongbirdKey>();

		let voice_guild_arc = data_lock
			.clone_expect::<VoiceGuilds>()
			.entry(guild_id)
			.or_default()
			.clone();

		let pool = data_lock.clone_expect::<Pool>();
		let volume = match play_style {
			PlayStyle::Clip => Config::get_volume_clip(&pool, &guild_id).await,
			PlayStyle::Play => Config::get_volume_play(&pool, &guild_id).await,
		}
		.map_err(|e| error!("Unable to get volume: {:?}", e))
		.ok()
		.flatten()
		.unwrap_or(0.5);

		let keys = data_lock.clone_expect::<Keys>();

		(songbird, voice_guild_arc, volume, keys)
	};

	debug!("Dropped lock for play");

	if let Some(call) = songbird.get(guild_id) {
		debug!("Fetching audio source");

		let result = match play_style {
			PlayStyle::Clip => match clip_source(args.search.as_ref()).await {
				Ok(clip) => {
					if play_input(
						play_style,
						call.clone(),
						voice_guild_arc,
						clip,
						volume,
						play_index,
					)
					.await
					{
						Ok(format!("Playing {}", args.search))
					} else {
						Ok(format!("Error playing {}", args.search))
					}
				}
				Err(e) => Err(e),
			},
			PlayStyle::Play => {
				match play_sources(keys, &args.search, play_index.is_none(), move |input| {
					let call = call.clone();
					let voice_guild_arc = voice_guild_arc.clone();

					async move {
						play_input(play_style, call, voice_guild_arc, input, volume, play_index)
							.await;
					}
				})
				.await
				{
					Ok(info) => {
						use std::fmt::Write;

						let title = info.title.as_deref().unwrap_or(&args.search);

						let mut response = String::from("Queuing");

						if info.count != 1 {
							write!(response, " {} clips from", info.count).unwrap();
						}

						match info.url {
							Some(url) => write!(response, " [{}]({})", title, url),
							None => write!(response, " {}", title),
						}
						.unwrap();

						if let Some(duration) = info.duration {
							response.push_str(" (");
							write_duration(&mut response, duration).unwrap();
							response.push(')');
						}

						Ok(response)
					}
					Err(e) => Err(e),
				}
			}
		};

		debug!("Finished fetching audio source");

		match result {
			Ok(response) => Ok(response.into()),
			Err(e) => {
				error!("Error playing audio: {}", e);
				Err(match e {
					AudioError::Songbird(_) => "Playback error".into(),
					AudioError::UnsupportedUrl => {
						format!("Unsupported URL: {}", &args.search).into()
					}
					AudioError::MultipleClip(clip_a, clip_b) => format!(
						"Multiple clips matching {} found. Please be more specific.\n\
						> {}\n\
						> {}\n\
						> ...",
						&args.search,
						clip_a.to_string_lossy(),
						clip_b.to_string_lossy()
					)
					.into(),
					AudioError::NotFound => format!("Clip {} not found", &args.search).into(),
					AudioError::Spotify => "Error reading from Spotify".into(),
					AudioError::YoutubePlaylist => "Error reading youtube playlist".into(),
					AudioError::PlaylistNotAllowed => {
						"A playlist is not allowed in this context".into()
					}
				})
			}
		}
	} else {
		Err("Not in a voice channel".into())
	}
}

async fn queue_input(
	call: Arc<Mutex<Call>>,
	input: Input,
	volume: f32,
	play_index: Option<usize>,
) -> bool {
	let (mut track, _handle) = create_player(input);
	track.set_volume(volume);

	let mut lock = call.lock().await;

	lock.enqueue(track);

	if let Some(index) = play_index {
		if index < lock.queue().len() {
			lock.queue().modify_queue(|q| {
				let v = q.pop_back().unwrap();

				if index == 0 {
					q.front().map(|f| f.handle().pause());
					let _ = v.handle().play();
				}

				q.insert(index, v);
			})
		}
	}

	true
}

async fn immediate_input(
	call: Arc<Mutex<Call>>,
	voice_guild_arc: ArcRw<VoiceGuild>,
	input: Input,
	volume: f32,
) -> bool {
	let (mut track, handle) = create_player(input);
	track.set_volume(volume);

	call.lock().await.play(track);
	voice_guild_arc
		.write()
		.await
		.add_audio(handle, volume)
		.is_ok()
}

async fn play_input(
	play_style: PlayStyle,
	call: Arc<Mutex<Call>>,
	voice_guild_arc: ArcRw<VoiceGuild>,
	input: Input,
	volume: f32,
	play_index: Option<usize>,
) -> bool {
	match play_style {
		PlayStyle::Clip => immediate_input(call, voice_guild_arc, input, volume).await,
		PlayStyle::Play => queue_input(call, input, volume, play_index).await,
	}
}
