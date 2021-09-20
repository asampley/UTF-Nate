use log::{debug, error};

use serenity::client::Context;
use serenity::model::prelude::GuildId;
use serenity::prelude::Mutex;

use songbird::create_player;
use songbird::input::Input;
use songbird::Call;
use songbird::SongbirdKey;

use std::sync::Arc;

use crate::audio::{clip_source, play_sources};
use crate::audio::{AudioError, PlayStyle};
use crate::configuration::Config;
use crate::data::{ArcRw, Keys, VoiceGuild, VoiceGuilds};
use crate::util::*;

pub async fn play(
	ctx: &Context,
	play_style: PlayStyle,
	path: Option<&str>,
	guild_id: Option<GuildId>,
	play_index: Option<usize>,
) -> Result<Response, Response> {
	let path = path.ok_or("Must provide a source")?;
	let guild_id = guild_id.ok_or("This command is only available in guilds")?;

	let (songbird, voice_guild_arc, volume, keys) = {
		debug!("Acquiring lock for play");

		let data_lock = ctx.data.read().await;

		debug!("Acquired lock for play");

		let songbird = data_lock.clone_expect::<SongbirdKey>();

		let voice_guild_arc = data_lock
			.clone_expect::<VoiceGuilds>()
			.entry(guild_id)
			.or_default()
			.clone();

		let volume = data_lock
			.clone_expect::<Config>()
			.read()
			.await
			.guilds
			.get(&guild_id)
			.and_then(|c| match play_style {
				PlayStyle::Clip => c.volume_clip,
				PlayStyle::Play => c.volume_play,
			})
			.unwrap_or(0.5);

		let keys = data_lock.clone_expect::<Keys>();

		(songbird, voice_guild_arc, volume, keys)
	};

	debug!("Dropped lock for play");

	if let Some(call) = songbird.get(guild_id) {
		debug!("Fetching audio source");

		let result = match play_style {
			PlayStyle::Clip => match clip_source(&path).await {
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
						Ok(format!("Playing {}", path))
					} else {
						Ok(format!("Error playing {}", path))
					}
				}
				Err(e) => Err(e),
			},
			PlayStyle::Play => {
				match play_sources(keys, &path, play_index.is_none(), move |input| {
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
						let title = info.title.as_deref().unwrap_or(path);

						Ok(format!(
							"Queuing {} {}",
							match info.count {
								1 => "".to_string(),
								count => format!("{} clips from", count),
							},
							match info.url {
								Some(url) => format!("[{}]({})", title, url),
								None => format!("{}", title),
							}
						))
					}
					Err(e) => Err(e),
				}
			}
		};

		debug!("Finished fetching audio source");

		match result {
			Ok(response) => Ok(response.into()),
			Err(e) => {
				error!("Error playing audio: {:?}", e);
				Err(match e {
					AudioError::Songbird(_) => "Playback error".into(),
					AudioError::UnsupportedUrl => format!("Unsupported URL: {}", path).into(),
					AudioError::MultipleClip => format!(
						"Multiple clips matching {} found. Please be more specific.",
						path
					)
					.into(),
					AudioError::NotFound => format!("Clip {} not found", path).into(),
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

async fn play_input(
	play_style: PlayStyle,
	call: Arc<Mutex<Call>>,
	voice_guild_arc: ArcRw<VoiceGuild>,
	input: Input,
	volume: f32,
	play_index: Option<usize>,
) -> bool {
	let (mut track, handle) = create_player(input);
	track.set_volume(volume);

	match play_style {
		PlayStyle::Clip => {
			call.lock().await.play(track);
			voice_guild_arc
				.write()
				.await
				.add_audio(handle, volume)
				.is_ok()
		}
		PlayStyle::Play => {
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
	}
}
