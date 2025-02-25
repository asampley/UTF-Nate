use itertools::Itertools;

use tracing::error;

use serde::{Deserialize, Serialize};

use songbird::tracks::ControlError;
use songbird::SongbirdKey;

use std::fs::read_dir;

use crate::audio::{PlayStyle, CLIP_PATH};
use crate::commands::{BotState, Source};
use crate::data::VoiceGuilds;
use crate::{util::*, StorageKey};

#[cfg(feature = "http-interface")]
pub mod http;
pub mod poise;

pub const fn volume_help() -> &'static str {
	include_str!("help/volume.md")
}

pub const fn volume_get_help() -> &'static str {
	include_str!("help/volume_get.md")
}

pub const fn volume_play_help() -> &'static str {
	include_str!("help/volume_play.md")
}

pub const fn volume_clip_help() -> &'static str {
	include_str!("help/volume_clip.md")
}

pub const fn volume_now_help() -> &'static str {
	include_str!("help/volume_now.md")
}

pub const fn list_help() -> &'static str {
	include_str!("help/list.md")
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VolumeSetArgs {
	volume: Option<f32>,
}

#[derive(Debug)]
pub enum VolumeMode {
	ConfigAllStyles,
	Config(PlayStyle, Option<f32>),
	Current(Option<f32>),
}

#[tracing::instrument(level = "info", ret)]
pub async fn list(path: Option<&str>) -> Result<Response, Response> {
	let dir = sandboxed_join(&CLIP_PATH, path.unwrap_or("")).ok_or("Invalid directory")?;

	let dir_iter = read_dir(dir)
		.inspect_err(|reason| error!("Unable to read directory: {:?}", reason))
		.map_err(|_| "Invalid directory")?;

	let message = dir_iter
		.filter_map(|e| e.inspect_err(|e| error!("{:?}", e)).ok())
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
		.map(|(f, t)| f + if t.is_dir() { "/" } else { "" })
		.fold("".to_owned(), |acc, s| acc + "\n" + &s);

	Ok(("```\n".to_owned() + &message + "\n```").into())
}

#[tracing::instrument(level = "info", ret, skip(state))]
pub async fn volume(
	state: &BotState,
	source: &Source,
	mode: VolumeMode,
) -> Result<Response, Response> {
	let guild_id = source
		.guild_id
		.ok_or("This command is only available in guilds")?;

	let data_lock = state.data.read().await;

	match mode {
		VolumeMode::ConfigAllStyles => {
			let storage = data_lock.get_expect::<StorageKey>();

			Ok(format!(
				"Play volume: {}\nClip volume: {}",
				storage
					.get_volume_play(guild_id)
					.await
					.inspect_err(|e| error!("Unable to retrieve volume: {:?}", e))
					.map_err(|_| "Unable to retrieve volume")?
					.unwrap_or(0.5),
				storage
					.get_volume_clip(guild_id)
					.await
					.inspect_err(|e| error!("Unable to retrieve volume: {:?}", e))
					.map_err(|_| "Unable to retrieve volume")?
					.unwrap_or(0.5)
			)
			.into())
		}
		VolumeMode::Config(style, None) => {
			let storage = data_lock.get_expect::<StorageKey>();

			Ok(match style {
				PlayStyle::Clip => format!(
					"Clip volume: {}",
					storage
						.get_volume_clip(guild_id)
						.await
						.inspect_err(|e| error!("Unable to retrieve volume: {:?}", e))
						.map_err(|_| "Unable to retrieve volume")?
						.unwrap_or(0.5),
				),
				PlayStyle::Play => format!(
					"Play volume: {}",
					storage
						.get_volume_play(guild_id)
						.await
						.inspect_err(|e| error!("Unable to retrieve volume: {:?}", e))
						.map_err(|_| "Unable to retrieve volume")?
						.unwrap_or(0.5),
				),
			}
			.into())
		}
		VolumeMode::Config(_, Some(volume)) | VolumeMode::Current(Some(volume)) => {
			if !(0.0..=1.0).contains(&volume) {
				return Err("Volume must be between 0.0 and 1.0".into());
			}

			let style = if let VolumeMode::Config(style, _) = mode {
				style
			} else {
				PlayStyle::Play
			};

			let ret = match style {
				PlayStyle::Play => {
					let songbird = data_lock.clone_expect::<SongbirdKey>();

					for handle in songbird
						.get_or_insert(guild_id)
						.lock()
						.await
						.queue()
						.current_queue()
					{
						handle.set_volume(volume).or_else(|e| match e {
							ControlError::Finished => Ok(()),
							_ => Err("Error setting volume"),
						})?;

						if let VolumeMode::Current(_) = &mode {
							break;
						}
					}

					if let VolumeMode::Current(_) = &mode {
						Ok(format!("Play volume set to {} for current audio", volume).into())
					} else {
						Ok(format!("Play volume set to {}", volume).into())
					}
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

			if let VolumeMode::Config(_, _) = mode {
				let storage = data_lock.get_expect::<StorageKey>();

				match style {
					PlayStyle::Clip => storage.set_volume_clip(guild_id, volume).await,
					PlayStyle::Play => storage.set_volume_play(guild_id, volume).await,
				}
				.inspect_err(|e| error!("Error setting volume: {:?}", e))
				.map_err(|_| "Error setting volume")?;
			}

			ret
		}
		VolumeMode::Current(None) => {
			let songbird = data_lock.clone_expect::<SongbirdKey>();

			let volume = songbird
				.get_or_insert(guild_id)
				.lock()
				.await
				.queue()
				.current()
				.ok_or("No song currently playing")?
				.get_info()
				.await
				.map(|info| info.volume)
				.inspect_err(|e| error!("Error getting volume: {:?}", e))
				.map_err(|_| "Error getting volume")?;

			Ok(format!("Current volume set to {}", volume).into())
		}
	}
}
