use serde::{Deserialize, Serialize};

use serenity::http::Http;
use serenity::model::id::ChannelId;

use songbird::input::Input;
use songbird::tracks::Track;
use songbird::Call;
use songbird::SongbirdKey;

use tracing::{debug, error};

use std::sync::Arc;

use crate::audio::{get_inputs, move_queue, SearchSource};
use crate::audio::{AudioError, PlayStyle};
use crate::commands::{BotState, Source};
use crate::data::TrackMetadata;
use crate::data::{ArcRw, Keys, VoiceGuild, VoiceGuilds};
use crate::parser::Selection;
use crate::persistence::Storage;
use crate::util::write_duration;
use crate::util::{GetExpect, Response};
use crate::StorageKey;

#[cfg(feature = "http-interface")]
pub mod http;
pub mod poise;

pub const fn clip_help() -> &'static str {
	include_str!("help/clip.md")
}

pub const fn play_help() -> &'static str {
	include_str!("help/play.md")
}

pub const fn playnext_help() -> &'static str {
	include_str!("help/playnext.md")
}

pub const fn playnow_help() -> &'static str {
	include_str!("help/playnow.md")
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

		let storage = data_lock.clone_expect::<StorageKey>();
		let volume = match play_style {
			PlayStyle::Clip => storage.get_volume_clip(guild_id).await,
			PlayStyle::Play => storage.get_volume_play(guild_id).await,
		}
		.inspect_err(|e| error!("Unable to get volume: {:?}", e))
		.ok()
		.flatten()
		.unwrap_or(0.5);

		let keys = data_lock.clone_expect::<Keys>();

		(songbird, voice_guild_arc, volume, keys)
	};

	debug!("Dropped lock for play");

	match songbird.get(guild_id) {
		None => Err("Not in a voice channel".into()),
		Some(call) => {
			debug!("Fetching audio input");

			let search_location = Some(match play_style {
				PlayStyle::Clip => SearchSource::Local,
				PlayStyle::Play => SearchSource::Youtube,
			});

			let channel_id = source.channel_id;

			let result = match get_inputs(keys, &args.search, true, search_location).await {
				Ok(info) => {
					use std::fmt::Write;

					let mut lock = call.lock().await;

					let mut input_count = 0;

					for input in info.inputs {
						play_input(
							play_style,
							&mut lock,
							voice_guild_arc.clone(),
							channel_id.map(|id| (state.http.clone(), id)),
							input,
							volume,
						)
						.await;

						input_count += 1;
					}

					if let Some(play_index) = play_index {
						let start = lock.queue().len() - input_count;

						let _ = move_queue(
							&mut lock,
							Selection::from(start..=(start + input_count - 1)),
							play_index,
						)
						.await
						.inspect_err(|e| error!("{:?}", e));
					}

					let title = info.title.as_deref().unwrap_or(&args.search);

					let mut response = match play_style {
						PlayStyle::Clip => String::from("Playing"),
						PlayStyle::Play => String::from("Queued"),
					};

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

					if play_style == PlayStyle::Play {
						let start = match play_index {
							Some(i) => i,
							None => lock.queue().len() - input_count,
						};

						if input_count > 1 {
							write!(
								response,
								" at positions {} to {}",
								start,
								start + input_count - 1
							)
							.unwrap();
						} else {
							write!(response, " at position {}", start).unwrap();
						}
					}

					Ok(response)
				}
				Err(e) => Err(e),
			};

			debug!("Finished fetching audio source");

			match result {
				Ok(response) => Ok(response.into()),
				Err(e) => {
					error!("Error playing audio: {}", e);
					Err(match e {
						AudioError::Songbird(_)
						| AudioError::Symphonia(_)
						| AudioError::Metadata(_) => "Playback error".into(),
						AudioError::UnsupportedUrl => {
							format!("Unsupported URL: {}", &args.search).into()
						}
						AudioError::NotFound => format!("Clip {} not found", &args.search).into(),
						AudioError::Spotify => "Error reading from Spotify".into(),
						AudioError::YoutubePlaylist => "Error reading youtube playlist".into(),
						AudioError::PlaylistNotAllowed => {
							"A playlist is not allowed in this context".into()
						}
					})
				}
			}
		}
	}
}

async fn queue_input(
	call: &mut Call,
	respond: Option<(Arc<Http>, ChannelId)>,
	mut input: Input,
	volume: f32,
) -> bool {
	let aux_metadata = input
		.aux_metadata()
		.await
		.inspect_err(|e| error!("Unable to fetch metadata: {:?}", e))
		.ok();

	let track = Track::from(input).volume(volume);

	let handle = call.enqueue(track).await;

	if let Some(meta) = aux_metadata {
		handle.typemap().write().await.insert::<TrackMetadata>(meta);
	}

	if let Err(e) = VoiceGuild::add_error_handler(handle, respond) {
		error!("Error setting up error handler for track: {:?}", e);
		return false;
	}

	true
}

async fn immediate_input(
	call: &mut Call,
	voice_guild_arc: ArcRw<VoiceGuild>,
	input: Input,
	volume: f32,
) -> bool {
	let track = Track::from(input).volume(volume);

	let handle = call.play(track);
	voice_guild_arc
		.write()
		.await
		.add_audio(handle, volume)
		.is_ok()
}

async fn play_input(
	play_style: PlayStyle,
	call: &mut Call,
	voice_guild_arc: ArcRw<VoiceGuild>,
	respond: Option<(Arc<Http>, ChannelId)>,
	input: Input,
	volume: f32,
) -> bool {
	match play_style {
		PlayStyle::Clip => immediate_input(call, voice_guild_arc, input, volume).await,
		PlayStyle::Play => queue_input(call, respond, input, volume).await,
	}
}
