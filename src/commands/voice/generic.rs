use itertools::Itertools;

use tracing::error;

use serenity::model::prelude::GuildId;

use songbird::error::TrackError;
use songbird::SongbirdKey;

use std::fs::read_dir;
use std::path::Path;

use crate::audio::clip_path;
use crate::audio::PlayStyle;
use crate::configuration::Config;
use crate::data::VoiceGuilds;
use crate::util::*;
use crate::Pool;

#[derive(Debug)]
pub enum VolumeMode {
	ConfigAllStyles,
	Config(PlayStyle, Option<f32>),
	Current(Option<f32>),
}

#[tracing::instrument(level = "info", ret)]
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

#[tracing::instrument(level = "info", ret, skip(ctx))]
pub async fn volume(
	ctx: &Context<'_>,
	guild_id: Option<GuildId>,
	mode: VolumeMode,
) -> Result<Response, Response> {
	let guild_id = guild_id.ok_or("This command is only available in guilds")?;

	let data_lock = ctx.discord().data.read().await;

	match mode {
		VolumeMode::ConfigAllStyles => {
			let pool = data_lock.clone_expect::<Pool>();

			Ok(format!(
				"Play volume: {}\nClip volume: {}",
				Config::get_volume_play(&pool, &guild_id)
					.await
					.map_err(|e| {
						error!("Unable to retrieve volume: {:?}", e);
						"Unable to retrieve volume"
					})?
					.unwrap_or(0.5),
				Config::get_volume_clip(&pool, &guild_id)
					.await
					.map_err(|e| {
						error!("Unable to retrieve volume: {:?}", e);
						"Unable to retrieve volume"
					})?
					.unwrap_or(0.5)
			)
			.into())
		}
		VolumeMode::Config(style, None) => {
			let pool = data_lock.clone_expect::<Pool>();

			Ok(match style {
				PlayStyle::Clip => format!(
					"Clip volume: {}",
					Config::get_volume_clip(&pool, &guild_id)
						.await
						.map_err(|e| {
							error!("Unable to retrieve volume: {:?}", e);
							"Unable to retrieve volume"
						})?
						.unwrap_or(0.5),
				),
				PlayStyle::Play => format!(
					"Play volume: {}",
					Config::get_volume_play(&pool, &guild_id)
						.await
						.map_err(|e| {
							error!("Unable to retrieve volume: {:?}", e);
							"Unable to retrieve volume"
						})?
						.unwrap_or(0.5),
				),
			}
			.into())
		}
		VolumeMode::Config(_, Some(volume)) | VolumeMode::Current(Some(volume)) => {
			if !(volume >= 0.0 && volume <= 1.0) {
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
						match handle
							.set_volume(volume)
							.err()
							.filter(|e| e == &TrackError::Finished)
						{
							Some(_) => return Err("Error setting volume".into()),
							None => (),
						}

						if let VolumeMode::Current(_) = &mode {
							break;
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

			if let VolumeMode::Config(_, _) = mode {
				let pool = data_lock.clone_expect::<Pool>();

				match style {
					PlayStyle::Clip => Config::set_volume_clip(&pool, &guild_id, volume).await,
					PlayStyle::Play => Config::set_volume_play(&pool, &guild_id, volume).await,
				}
				.map_err(|e| {
					error!("Error setting volume: {:?}", e);
					"Error setting volume"
				})?;
			}

			return ret;
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
				.map_err(|e| {
					error!("Error getting volume: {:?}", e);
					"Error getting volume"
				})?;

			Ok(format!("Current volume set to {}", volume).into())
		}
	}
}
