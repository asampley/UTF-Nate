use itertools::Itertools;

use log::{debug, error};

use serenity::client::Context;
use serenity::model::prelude::{GuildId, UserId};
use serenity::prelude::Mutex;

use songbird::create_player;
use songbird::error::TrackError;
use songbird::input::Input;
use songbird::Call;
use songbird::SongbirdKey;

use std::fs::read_dir;
use std::path::Path;
use std::sync::Arc;

use crate::audio::{clip_path, clip_source, play_sources};
use crate::audio::{AudioError, PlayStyle};
use crate::configuration::{write_config_eprintln, Config};
use crate::data::{ArcRw, Keys, VoiceGuild, VoiceGuilds};
use crate::util::*;

pub async fn summon(
	ctx: &Context,
	guild_id: Option<GuildId>,
	user_id: UserId,
) -> Result<Response, Response> {
	let guild_id = guild_id.ok_or("This command is only available in guilds")?;
	let guild = guild_id
		.to_guild_cached(&ctx.cache)
		.await
		.ok_or("Internal bot error")?;

	let channel_id = guild
		.voice_states
		.get(&user_id)
		.and_then(|voice_state| voice_state.channel_id);

	let connect_to = channel_id.ok_or("Not in a voice channel")?;

	let songbird = ctx.data.read().await.clone_expect::<SongbirdKey>();
	let (_call, join_result) = songbird.join(guild_id, connect_to).await;

	match join_result {
		Ok(()) => Ok("Joined channel".into()),
		Err(_) => Err("Error joining the channel".into()),
	}
}

pub async fn banish(ctx: &Context, guild_id: Option<GuildId>) -> Result<Response, Response> {
	let guild_id = guild_id.ok_or("This command is only available in guilds")?;

	let songbird = ctx.data.read().await.clone_expect::<SongbirdKey>();

	{
		use songbird::error::JoinError::*;
		match songbird.remove(guild_id).await {
			Ok(()) => Ok("Left voice channel".into()),
			Err(e) => match e {
				NoCall => Err("Not in a voice channel".into()),
				_ => Err("Internal bot error".into()),
			},
		}
	}
}

pub async fn play(
	ctx: &Context,
	play_style: PlayStyle,
	path: Option<&str>,
	guild_id: Option<GuildId>,
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
					if play_input(play_style, call.clone(), voice_guild_arc, clip, volume).await {
						Ok(format!("Playing {}", path))
					} else {
						Ok(format!("Error playing {}", path))
					}
				}
				Err(e) => Err(e),
			},
			PlayStyle::Play => {
				match play_sources(keys, &path, move |input| {
					let call = call.clone();
					let voice_guild_arc = voice_guild_arc.clone();

					async move { play_input(play_style, call, voice_guild_arc, input, volume).await }
				})
				.await
				{
					Ok(count) => Ok(match count {
						1 => format!("Queuing clip from {}", path),
						count => format!("Queuing {} clips from {}", count, path),
					}),
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
					AudioError::NoClip => format!("Clip {} not found", path).into(),
					AudioError::Spotify => "Error reading from Spotify".into(),
					AudioError::YoutubePlaylist => "Error reading youtube playlist".into(),
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
			call.lock().await.enqueue(track);
			true
		}
	}
}

pub async fn list(path: Option<&str>) -> Result<Response, Response> {
	let dir = clip_path().join(Path::new(match path {
		None => "",
		Some(ref path) => path,
	}));

	let dir = dir.canonicalize().map_err(|_| "Invalid directory")?;

	if !sandboxed_exists(&clip_path(), &dir) {
		return Err("Invalid directory".into());
	}

	match read_dir(dir) {
		Err(reason) => {
			error!("Unable to read directory: {:?}", reason);
			return Err("Invalid directory".into());
		}
		Ok(dir_iter) => {
			let message = dir_iter
				.filter_map(|e| e.ok())
				.map(|e| {
					(
						e.path()
							.file_stem()
							.and_then(|f| f.to_str())
							.map(|f| f.to_owned()),
						e.file_type(),
					)
				})
				.filter(|(f, t)| f.is_some() && t.is_ok())
				.map(|(f, t)| (f.unwrap(), t.unwrap()))
				.sorted_by(|(f0, t0), (f1, t1)| {
					(!t0.is_dir(), f0.to_lowercase()).cmp(&(!t1.is_dir(), f1.to_lowercase()))
				})
				.map(|(f, t)| format!("{: <20}", f + if t.is_dir() { "/" } else { "" }))
				.chunks(3)
				.into_iter()
				.map(|chunk| chunk.fold("".to_owned(), |acc, s| acc + &s))
				.fold("".to_owned(), |acc, s| acc + "\n" + &s);

			return Ok(("```\n".to_owned() + &message + "\n```").into());
		}
	}
}

pub async fn volume(
	ctx: &Context,
	style: Option<PlayStyle>,
	guild_id: Option<GuildId>,
	volume: Option<f32>,
) -> Result<Response, Response> {
	let guild_id = guild_id.ok_or("This command is only available in guilds")?;

	let data_lock = ctx.data.read().await;

	match (style, volume) {
		(None, None) => {
			let config_arc = data_lock.clone_expect::<Config>();
			let config = config_arc.read().await;

			let guild_config = config.guilds.get(&guild_id);
			Ok(format!(
				"Play volume: {}\nClip volume: {}",
				guild_config.and_then(|c| c.volume_play).unwrap_or(0.5),
				guild_config.and_then(|c| c.volume_clip).unwrap_or(0.5),
			)
			.into())
		}
		(Some(style), None) => {
			let config_arc = data_lock.clone_expect::<Config>();
			let config = config_arc.read().await;

			let guild_config = config.guilds.get(&guild_id);
			Ok(match style {
				PlayStyle::Clip => format!(
					"Clip volume: {}",
					guild_config.and_then(|c| c.volume_clip).unwrap_or(0.5)
				),
				PlayStyle::Play => format!(
					"Play volume: {}",
					guild_config.and_then(|c| c.volume_play).unwrap_or(0.5)
				),
			}
			.into())
		}
		(None, Some(_volume)) => Err(
			"Please specify either \"play\" or \"clip\" to set the volume for each command".into(),
		),
		(Some(style), Some(volume)) => {
			if !(volume >= 0.0 || volume <= 1.0) {
				return Err("Volume must be between 0.0 and 1.0".into());
			} else {
				let ret = match style {
					PlayStyle::Play => {
						let songbird = data_lock.clone_expect::<SongbirdKey>();

						for handle in songbird
							.get_or_insert(guild_id.into())
							.lock()
							.await
							.queue()
							.current_queue()
						{
							match handle
								.set_volume(volume)
								.err()
								.filter(|e| e == &TrackError::Finished)
							{
								Some(_) => return Err("Error setting volume".into()),
								None => (),
							}
						}

						Ok(format!("Play volume set to {}", volume).into())
					}
					PlayStyle::Clip => data_lock
						.clone_expect::<VoiceGuilds>()
						.entry(guild_id)
						.or_default()
						.clone()
						.write()
						.await
						.set_volume(volume)
						.map(|_| format!("Clip volume set to {}", volume).into())
						.map_err(|_| "Error setting volume".into()),
				};

				let config_arc = data_lock.clone_expect::<Config>();
				let mut config = config_arc.write().await;

				let entry = config.guilds.entry(guild_id).or_default();
				match style {
					PlayStyle::Clip => entry.volume_clip = Some(volume),
					PlayStyle::Play => entry.volume_play = Some(volume),
				}

				write_config_eprintln(Path::new("config.json"), &*config);

				return ret;
			}
		}
	}
}

pub async fn stop(ctx: &Context, guild_id: Option<GuildId>) -> Result<Response, Response> {
	let guild_id = guild_id.ok_or("This command is only available in guilds")?;

	let lock = ctx.data.read().await;

	if let Some(voice_guild) = lock.clone_expect::<VoiceGuilds>().get(&guild_id) {
		voice_guild.write().await.stop()
	}

	ctx.data
		.read()
		.await
		.clone_expect::<SongbirdKey>()
		.get_or_insert(guild_id.into())
		.lock()
		.await
		.queue()
		.stop();

	Ok("Cleared queue and stopped playing".into())
}

pub async fn skip(
	ctx: &Context,
	guild_id: Option<GuildId>,
	skip_count: Option<usize>,
) -> Result<Response, Response> {
	let guild_id = guild_id.ok_or("This command is only available in guilds")?;

	let call = ctx
		.data
		.read()
		.await
		.clone_expect::<SongbirdKey>()
		.get_or_insert(guild_id.into())
		.clone();

	let call = call.lock().await;

	let queue = call.queue();

	let result = if let Some(count) = skip_count {
		queue
			.modify_queue(|deque| {
				(0..count)
					.filter_map(|_| deque.pop_front())
					.fuse()
					.map(|queued| queued.stop())
					.fold_ok(0, |acc, _| acc + 1)
			})
			.and_then(|count| queue.resume().map(|_| count))
	} else {
		queue.skip().map(|_| 1)
	};

	result
		.map(|count| match count {
			1 => "Skipped current clip".into(),
			_ => format!("Skipped {} clips", count).into(),
		})
		.map_err(|e| {
			error!("{:?}", e);
			"Error skipping clips".into()
		})
}

pub async fn pause(ctx: &Context, guild_id: Option<GuildId>) -> Result<Response, Response> {
	let guild_id = guild_id.ok_or("This command is only available in guilds")?;

	ctx.data
		.read()
		.await
		.clone_expect::<SongbirdKey>()
		.get_or_insert(guild_id.into())
		.lock()
		.await
		.queue()
		.pause()
		.map(|_| "Pausing current clip".into())
		.map_err(|e| {
			error!("{:?}", e);
			"Error pausing clip".into()
		})
}

pub async fn unpause(ctx: &Context, guild_id: Option<GuildId>) -> Result<Response, Response> {
	let guild_id = guild_id.ok_or("This command is only available in guilds")?;

	ctx.data
		.read()
		.await
		.clone_expect::<SongbirdKey>()
		.get_or_insert(guild_id.into())
		.lock()
		.await
		.queue()
		.resume()
		.map(|_| "Unpausing current clip".into())
		.map_err(|e| {
			error!("{:?}", e);
			"Error unpausing clip".into()
		})
}
