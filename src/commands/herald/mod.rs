use tap::TapFallible;
use tracing::error;

use serde::{Deserialize, Serialize};

use crate::audio::{find_clip, FindClip};
use crate::commands::{BotState, Source};
use crate::configuration::Config;
use crate::util::{GetExpect, Response};
use crate::Pool;

use IntroOutroMode::*;

#[cfg(feature = "http-interface")]
pub mod http;
pub mod poise;

pub const fn intro_help() -> &'static str {
	include_str!("intro.md")
}

pub const fn outro_help() -> &'static str {
	include_str!("outro.md")
}

pub const fn introbot_help() -> &'static str {
	include_str!("introbot.md")
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
		Some(clip) => match find_clip(clip.as_ref()) {
			FindClip::One(clip) => Some(clip),
			FindClip::Multiple(clip_a, clip_b) => {
				return Err(format!(
					"Multiple clips matching {} found. Please be more specific.\n\
					> {}\n\
					> {}\n\
					> ...",
					clip,
					clip_a.to_string_lossy(),
					clip_b.to_string_lossy()
				)
				.into())
			}
			FindClip::None => return Ok(format!("Clip {} not found", clip).into()),
		},
		None => None,
	};

	let data_lock = state.data.read().await;
	let pool = data_lock.clone_expect::<Pool>();

	match clip {
		None => {
			let clip = match mode {
				Intro => Config::get_intro(&pool, &source.user_id).await,
				Outro => Config::get_outro(&pool, &source.user_id).await,
			}
			.tap_err(|e| error!("Unable to fetch user data: {:?}", e))
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
				Intro => Config::set_intro(&pool, &source.user_id, clip).await,
				Outro => Config::set_outro(&pool, &source.user_id, clip).await,
			}
			.tap_err(|e| error!("Unable to write user data: {:?}", e))
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
		Some(clip) => match find_clip(clip.as_ref()) {
			FindClip::One(clip) => Some(clip),
			FindClip::Multiple(clip_a, clip_b) => {
				return Err(format!(
					"Multiple clips matching {} found. Please be more specific.\n\
					> {}\n\
					> {}\n\
					> ...",
					clip,
					clip_a.to_string_lossy(),
					clip_b.to_string_lossy()
				)
				.into())
			}
			FindClip::None => return Err(format!("Clip {} not found", clip).into()),
		},
		None => None,
	};

	let data_lock = state.data.read().await;
	let pool = data_lock.clone_expect::<Pool>();

	match clip {
		Some(clip) => {
			let clip = &clip.to_str().ok_or_else(|| {
				error!("Could not encode clip as unicode");
				format!(
					"Unable to set intro to {} due to unicode encoding issue",
					clip.to_string_lossy()
				)
			})?;

			Config::set_bot_intro(&pool, &guild_id, clip)
				.await
				.tap_err(|e| error!("Unable to set bot intro: {:?}", e))
				.map_err(|_| "Unable to set bot intro")?;

			Ok(format!("Set bot intro to {}", clip).into())
		}
		None => {
			let intro = Config::get_bot_intro(&pool, &guild_id)
				.await
				.tap_err(|e| error!("Unable to retrieve bot intro: {:?}", e))
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
