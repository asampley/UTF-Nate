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

use crate::configuration::{write_config_eprintln, Config};
use crate::data::{ArcRw, Keys, VoiceGuild, VoiceGuilds};
use crate::util::*;
use crate::voice::{clip_path, clip_source, play_sources, sandboxed_exists};
use crate::voice::{AudioError, PlayStyle};

pub async fn summon(ctx: &Context, guild_id: Option<GuildId>, user_id: UserId) -> String {
	let guild_id = unwrap_or_ret!(
		guild_id,
		"This command is only available in guilds".to_string()
	);
	let guild = unwrap_or_ret!(
		guild_id.to_guild_cached(&ctx.cache).await,
		"Internal bot error".to_string()
	);

	let channel_id = guild
		.voice_states
		.get(&user_id)
		.and_then(|voice_state| voice_state.channel_id);

	let connect_to = unwrap_or_ret!(channel_id, "Not in a voice channel".to_string());

	let songbird = ctx.data.read().await.clone_expect::<SongbirdKey>();

	match songbird.join(guild_id, connect_to).await.1 {
		Ok(()) => "Joined channel",
		Err(_) => "Error joining the channel",
	}
	.to_string()
}

pub async fn banish(ctx: &Context, guild_id: Option<GuildId>) -> String {
	let guild_id = unwrap_or_ret!(
		guild_id,
		"This command is only available in guilds".to_string()
	);

	let songbird = ctx.data.read().await.clone_expect::<SongbirdKey>();

	{
		use songbird::error::JoinError::*;
		match songbird.remove(guild_id).await {
			Ok(()) => "Left voice channel",
			Err(e) => match e {
				NoCall => "Not in a voice channel",
				_ => "Internal bot error",
			},
		}
		.to_string()
	}
}

pub async fn play(
	ctx: &Context,
	play_style: PlayStyle,
	path: Option<&str>,
	guild_id: Option<GuildId>,
) -> String {
	let path = unwrap_or_ret!(path, "Must provide a source".to_string());
	let guild_id = unwrap_or_ret!(
		guild_id,
		"This command is only available in guilds".to_string()
	);

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
			Ok(response) => response,
			Err(e) => {
				error!("Error playing audio: {:?}", e);
				match e {
					AudioError::Songbird(_) => "Playback error".to_string(),
					AudioError::UnsupportedUrl => format!("Unsupported URL: {}", path),
					AudioError::MultipleClip => format!(
						"Multiple clips matching {} found. Please be more specific.",
						path
					),
					AudioError::NoClip => format!("Clip {} not found", path),
					AudioError::Spotify => "Error reading from Spotify".to_string(),
					AudioError::YoutubePlaylist => "Error reading youtube playlist".to_string(),
				}
			}
		}
	} else {
		"Not in a voice channel".to_string()
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

pub async fn list(path: Option<&str>) -> String {
	let dir = clip_path().join(Path::new(match path {
		None => "",
		Some(ref path) => path,
	}));

	let dir = match dir.canonicalize() {
		Ok(dir) => dir,
		Err(_) => return "Invalid directory".to_string(),
	};

	if !sandboxed_exists(&clip_path(), &dir) {
		return "Invalid directory".to_string();
	}

	match read_dir(dir) {
		Err(reason) => {
			error!("Unable to read directory: {:?}", reason);
			return "Invalid directory".to_string();
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

			return "```\n".to_owned() + &message + "\n```";
		}
	}
}

pub async fn volume(
	ctx: &Context,
	style: Option<PlayStyle>,
	guild_id: Option<GuildId>,
	volume: Option<f32>,
) -> String {
	let guild_id = unwrap_or_ret!(
		guild_id,
		"This command is only available in guilds".to_string()
	);

	let style = unwrap_or_ret!(
		style,
		"Please specify either \"play\" or \"clip\" to set the volume for each command".to_string()
	);

	let volume = unwrap_or_ret!(
		volume,
		"Please specify a volume between 0.0 and 1.0".to_string()
	);

	if !(volume >= 0.0 || volume <= 1.0) {
		return "Volume must be between 0.0 and 1.0".to_string();
	}

	let data_lock = ctx.data.read().await;

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
					Some(_) => return "Error setting volume".to_string(),
					None => (),
				}
			}

			format!("Play volume set to {}", volume)
		}
		PlayStyle::Clip => {
			match data_lock
				.clone_expect::<VoiceGuilds>()
				.entry(guild_id)
				.or_default()
				.clone()
				.write()
				.await
				.set_volume(volume)
			{
				Ok(_) => format!("Clip volume set to {}", volume),
				Err(_) => "Error setting volume".to_string(),
			}
		}
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

pub async fn stop(ctx: &Context, guild_id: Option<GuildId>) -> String {
	let guild_id = unwrap_or_ret!(
		guild_id,
		"This command is only available in guilds".to_string()
	);

	ctx.data
		.read()
		.await
		.clone_expect::<SongbirdKey>()
		.get_or_insert(guild_id.into())
		.lock()
		.await
		.stop();

	"Cleared queue and stopped playing".to_string()
}

pub async fn skip(ctx: &Context, guild_id: Option<GuildId>) -> String {
	let guild_id = unwrap_or_ret!(
		guild_id,
		"This command is only available in guilds".to_string()
	);

	match ctx
		.data
		.read()
		.await
		.clone_expect::<SongbirdKey>()
		.get_or_insert(guild_id.into())
		.lock()
		.await
		.queue()
		.skip()
	{
		Ok(_) => "Skipping current clip".to_string(),
		Err(e) => {
			error!("{:?}", e);
			"Error skipping clip".to_string()
		}
	}
}
