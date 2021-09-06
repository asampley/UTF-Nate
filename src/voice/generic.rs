use itertools::Itertools;

use log::{debug, error};

use serenity::client::Context;
use serenity::model::prelude::{GuildId, UserId};

use songbird::create_player;
use songbird::error::TrackError;
use songbird::SongbirdKey;

use std::fs::read_dir;
use std::path::Path;

use crate::configuration::{write_config_eprintln, Config};
use crate::data::VoiceGuilds;
use crate::util::*;

use crate::voice::{audio_source, clip_path, sandboxed_exists};
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

	let (songbird, voice_guild_arc, volume) = {
		debug!("Acquiring lock for play");

		let data_lock = ctx.data.read().await;

		debug!("Acquired lock for play");

		let songbird = data_lock.clone_expect::<SongbirdKey>();

		let voice_guild_arc = data_lock
			.clone_expect::<VoiceGuilds>()
			.write()
			.await
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

		(songbird, voice_guild_arc, volume)
	};

	debug!("Dropped lock for play");

	if let Some(call) = songbird.get(guild_id) {
		debug!("Fetching audio source");

		let source = match audio_source(&path, play_style).await {
			Ok(input) => input,
			Err(e) => {
				error!("Error playing audio: {:?}", e);
				return match e {
					AudioError::Songbird(_) => "Playback error".to_string(),
					AudioError::UnsupportedUrl => format!("Unsupported URL: {}", path),
					AudioError::MultipleClip => format!(
						"Multiple clips matching {} found. Please be more specific.",
						path
					),
					AudioError::NoClip => format!("Clip {} not found", path),
					AudioError::Spotify => "Spotify support coming soon? \u{1f91e}".to_string(),
				};
			}
		};

		debug!("Finished fetching audio source");

		let (mut track, handle) = create_player(source);
		track.set_volume(volume);

		match play_style {
			PlayStyle::Clip => {
				call.lock().await.play(track);
				match voice_guild_arc.write().await.add_audio(handle, volume) {
					Ok(()) => format!("Playing {}", path),
					Err(_) => format!("Error playing {}", path),
				}
			}
			PlayStyle::Play => {
				call.lock().await.enqueue(track);
				format!("Queued {}", path)
			}
		}
	} else {
		"Not in a voice channel".to_string()
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

pub async fn volume(ctx: &Context, style: Option<PlayStyle>, guild_id: Option<GuildId>, volume: Option<f32>) -> String {
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
				match handle.set_volume(volume).err().filter(|e| e == &TrackError::Finished)
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
				.write()
				.await
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
