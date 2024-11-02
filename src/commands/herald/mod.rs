use tracing::error;

use serde::{Deserialize, Serialize};

use crate::audio::search_clips;
use crate::commands::{BotState, Source};
use crate::util::{GetExpect, Response};
use crate::StorageKey;

use IntroOutroMode::*;

#[cfg(feature = "http-interface")]
pub mod http;
pub mod poise;

pub const fn intro_help() -> &'static str {
	include_str!("help/intro.md")
}

pub const fn outro_help() -> &'static str {
	include_str!("help/outro.md")
}

pub const fn introbot_help() -> &'static str {
	include_str!("help/introbot.md")
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IntroOutroMode {
	Intro,
	Outro,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IntroOutroArgs {
	clip: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IntroBotArgs {
	clip: Option<String>,
}

impl IntroOutroMode {
	fn lowercase(&self) -> &'static str {
		match self {
			Intro => "intro",
			Outro => "outro",
		}
	}
}

#[tracing::instrument(level = "info", ret, skip(state))]
pub async fn intro_outro(
	state: &BotState,
	source: &Source,
	mode: IntroOutroMode,
	args: &IntroOutroArgs,
) -> Result<Response, Response> {
	let clip = match &args.clip {
		Some(clip) => {
			let mut clips = search_clips(clip.as_ref());

			if clips.is_empty() {
				return Ok(format!("Clip {} not found", clip).into());
			} else if clips.len() > 1 {
				return Err(format!(
					"Multiple clips matching {} found. Please be more specific.\n\
					> {}\n\
					> {}\n\
					> ...",
					clip,
					clips[0].to_string_lossy(),
					clips[1].to_string_lossy()
				)
				.into());
			} else {
				clips.pop()
			}
		}
		None => None,
	};

	let data_lock = state.data.read().await;
	let storage = data_lock.get_expect::<StorageKey>();

	match clip {
		None => {
			let clip = match mode {
				Intro => storage.get_intro(source.user_id).await,
				Outro => storage.get_outro(source.user_id).await,
			}
			.inspect_err(|e| error!("Unable to fetch user data: {:?}", e))
			.map_err(|_| "Unable to retrieve intro/outro")?;

			Ok(format!(
				"User {} is {}",
				mode.lowercase(),
				match clip {
					None => "default".to_owned(),
					Some(clip) => format!("\"{}\"", clip),
				}
			)
			.into())
		}
		Some(clip) => {
			let clip = &clip.to_str().ok_or_else(|| {
				error!("Could not encode clip as unicode");
				format!(
					"Unable to set intro to {} due to unicode encoding issue",
					clip.to_string_lossy()
				)
			})?;

			match mode {
				Intro => storage.set_intro(source.user_id, clip).await,
				Outro => storage.set_outro(source.user_id, clip).await,
			}
			.inspect_err(|e| error!("Unable to write user data: {:?}", e))
			.map_err(|_| "Unable to set intro/outro")?;

			Ok(format!("Set new {} to {}", mode.lowercase(), clip).into())
		}
	}
}

#[tracing::instrument(level = "info", ret, skip(state))]
pub async fn introbot(
	state: &BotState,
	source: &Source,
	args: &IntroBotArgs,
) -> Result<Response, Response> {
	let guild_id = source
		.guild_id
		.ok_or_else(|| "Groups and DMs not supported".to_string())?;

	let clip = match &args.clip {
		Some(clip) => {
			let mut clips = search_clips(clip.as_ref());

			if clips.is_empty() {
				return Ok(format!("Clip {} not found", clip).into());
			} else if clips.len() > 1 {
				return Err(format!(
					"Multiple clips matching {} found. Please be more specific.\n\
					> {}\n\
					> {}\n\
					> ...",
					clip,
					clips[0].to_string_lossy(),
					clips[1].to_string_lossy()
				)
				.into());
			} else {
				clips.pop()
			}
		}
		None => None,
	};

	let data_lock = state.data.read().await;
	let storage = data_lock.get_expect::<StorageKey>();

	match clip {
		Some(clip) => {
			let clip = &clip.to_str().ok_or_else(|| {
				error!("Could not encode clip as unicode");
				format!(
					"Unable to set intro to {} due to unicode encoding issue",
					clip.to_string_lossy()
				)
			})?;

			storage
				.set_bot_intro(guild_id, clip)
				.await
				.inspect_err(|e| error!("Unable to set bot intro: {:?}", e))
				.map_err(|_| "Unable to set bot intro")?;

			Ok(format!("Set bot intro to {}", clip).into())
		}
		None => {
			let intro = storage
				.get_bot_intro(guild_id)
				.await
				.inspect_err(|e| error!("Unable to retrieve bot intro: {:?}", e))
				.map_err(|_| "Unable to retrieve bot intro")?;

			Ok(format!(
				"Bot intro is {}",
				match intro {
					None => "default".to_owned(),
					Some(intro) => format!("\"{}\"", intro),
				}
			)
			.into())
		}
	}
}
