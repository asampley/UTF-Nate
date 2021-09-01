use itertools::Itertools;

use serenity::client::Context;
use serenity::model::prelude::{UserId, GuildId};

use songbird::create_player;
use songbird::SongbirdKey;
use songbird::error::TrackError;

use std::fs::read_dir;
use std::path::Path;

use crate::configuration::{Config, write_config};
use crate::data::VoiceGuilds;
use crate::util::*;

use crate::voice::{AudioError, clip_path, sandboxed_exists, audio_source};

pub enum PlayType {
	PlayNow,
	Queue,
}

pub async fn summon(ctx: &Context, guild_id: Option<GuildId>, user_id: UserId) -> String {
	let guild_id = unwrap_or_ret!(guild_id, "This command is only available in guilds".to_string());
	let guild = unwrap_or_ret!(
		guild_id.to_guild_cached(&ctx.cache).await,
		"Internal bot error".to_string()
	);

	let channel_id = guild
		.voice_states
		.get(&user_id)
		.and_then(|voice_state| voice_state.channel_id);

	let connect_to = unwrap_or_ret!(channel_id, "Not in a voice channel".to_string());

	let songbird = ctx
		.data
		.read()
		.await
		.clone_expect::<SongbirdKey>();

	match songbird.join(guild_id, connect_to).await.1 {
		Ok(()) => "Joined channel",
		Err(_) => "Error joining the channel",
	}.to_string()
}

pub async fn banish(ctx: &Context, guild_id: Option<GuildId>) -> String {
	let guild_id = unwrap_or_ret!(guild_id, "This command is only available in guilds".to_string());

	let songbird = ctx
		.data
		.read()
		.await
		.clone_expect::<SongbirdKey>();

	{
		use songbird::error::JoinError::*;
		match songbird.remove(guild_id).await {
			Ok(()) => "Left voice channel",
			Err(e) => match e {
				NoCall => "Not in a voice channel",
				_ => "Internal bot error",
			}
		}.to_string()
	}
}

pub async fn play(
	ctx: &Context,
	play_type: PlayType,
	path: Option<&str>,
	guild_id: Option<GuildId>,
) -> String {
	let path = unwrap_or_ret!(path, "Must provide a source".to_string());
	let guild_id = unwrap_or_ret!(guild_id, "This command is only available in guilds".to_string());

	{
		let data_lock = ctx.data.read().await;

		let songbird = data_lock
			.clone_expect::<SongbirdKey>();

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
			.and_then(|c| c.volume)
			.unwrap_or(0.5);

		let mut voice_guild = voice_guild_arc.write().await;

		if let Some(call) = songbird.get(guild_id) {
			let source = match audio_source(&path).await {
				Ok(input) => input,
				Err(e) => {
					eprintln!("Error playing audio: {:?}", e);
					return match e {
						AudioError::Songbird(_) => "Playback error".to_string(),
						AudioError::UnsupportedUrl => format!("Unsupported URL: {}", path), 
						AudioError::NoClip => format!("Clip {} not found", path),
						AudioError::Spotify => "Spotify support coming soon? \u{1f91e}".to_string(),
					}
				}
			};

			let (mut track, handle) = create_player(source);
			track.set_volume(volume);

			match play_type {
				PlayType::PlayNow => {
					call.lock().await.play(track);
					match voice_guild.add_audio(handle, volume) {
						Ok(()) => format!("Playing {}", path),
						Err(_) => format!("Error playing {}", path)
					}
				}
				PlayType::Queue => {
					call.lock().await.enqueue(track);
					format!("Queued {}", path)
				}
			}
		} else {
			"Not in a voice channel".to_string()
		}
	}
}

pub async fn list(
	path: Option<&str>,
) -> String {
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
			eprintln!("Unable to read directory: {:?}", reason);
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
	guild_id: Option<GuildId>,
	volume: Option<f32>,
) -> String {
	let guild_id = unwrap_or_ret!(guild_id, "This command is only available in guilds".to_string());
	let volume = unwrap_or_ret!(volume, "Please specify a volume between 0.0 and 1.0".to_string());

	if volume < 0.0 || volume > 1.0 {
		return "Volume must be between 0.0 and 1.0".to_string()
	}

	let data_lock = ctx.data.read().await;

	let songbird = data_lock.clone_expect::<SongbirdKey>();

	for handle in songbird.get_or_insert(guild_id.into()).lock().await.queue().current_queue() {
		if let Some(_) = handle.set_volume(volume).err().filter(|e| e == &TrackError::Finished) {
			return "Error setting volume".to_string();
		}
	}

	let ret = match data_lock
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
		Ok(_) => format!("Volume set to {}", volume),
		Err(_) => "Error setting volume".to_string(),
	};

	let config_arc = data_lock.clone_expect::<Config>();
	let mut config = config_arc.write().await;

	config.guilds.entry(guild_id).or_default().volume = Some(volume);

	{
		use crate::configuration::Result::*;
		match write_config(Path::new("config.json"), &*config) {
			Ok(()) => (),
			JsonError(reason) => eprintln!("Error writing config file: {:?}", reason),
			IoError(reason) => eprintln!("Error writing config file: {:?}", reason),
		}
	}

	return ret;
}

pub async fn stop(ctx: &Context, guild_id: Option<GuildId>) -> String {
	let guild_id = unwrap_or_ret!(guild_id, "This command is only available in guilds".to_string());

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
	let guild_id = unwrap_or_ret!(guild_id, "This command is only available in guilds".to_string());

	match ctx.data
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
			eprintln!("{:?}", e);
			"Error skipping clip".to_string()
		}
	}

}