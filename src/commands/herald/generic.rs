use tracing::error;

use serenity::client::Context;
use serenity::model::prelude::{GuildId, UserId};

use crate::audio::{find_clip, FindClip};
use crate::configuration::Config;
use crate::util::{GetExpect, Response};
use crate::Pool;

use super::IntroOutroMode::{self, *};

#[tracing::instrument(level = "info", ret, skip(ctx))]
pub async fn intro_outro(
	ctx: &Context,
	mode: IntroOutroMode,
	user_id: UserId,
	clip: Option<String>,
) -> Result<Response, Response> {
	let clip = match clip {
		Some(clip) => match find_clip(&clip) {
			FindClip::One(clip) => Some(clip),
			FindClip::Multiple(clip_a, clip_b) => {
				return Err(format!(
					"Multiple clips matching {} found. Please be more specific.\n\
					> {}\n\
					> {}\n\
					> ...",
					clip,
					clip_a,
					clip_b
				)
				.into())
			}
			FindClip::None => return Ok(format!("Clip {} not found", clip).into()),
		},
		None => None,
	};

	let data_lock = ctx.data.read().await;
	let pool = data_lock.clone_expect::<Pool>();

	match clip {
		None => {
			let clip = match mode {
				Intro => Config::get_intro(&pool, &user_id).await,
				Outro => Config::get_outro(&pool, &user_id).await,
			}
			.map_err(|e| {
				error!("Unable to fetch user data: {:?}", e);
				"Unable to retrieve intro/outro"
			})?;

			Ok(format!(
				"User {} is {}",
				mode.lowercase(),
				match clip {
					None => format!("default"),
					Some(clip) => format!("\"{}\"", clip),
				}
			)
			.into())
		}
		Some(clip) => {
			match mode {
				Intro => Config::set_intro(&pool, &user_id, &clip).await,
				Outro => Config::set_outro(&pool, &user_id, &clip).await,
			}
			.map_err(|e| {
				error!("Unable to write user data: {:?}", e);
				"Unable to set intro/outro"
			})?;

			Ok(format!("Set new {} to {}", mode.lowercase(), clip).into())
		}
	}
}

#[tracing::instrument(level = "info", ret, skip(ctx))]
pub async fn introbot(
	ctx: &Context,
	guild_id: Option<GuildId>,
	clip: Option<String>,
) -> Result<Response, Response> {
	let guild_id = guild_id.ok_or("Groups and DMs not supported".to_string())?;

	let clip = match clip {
		Some(clip) => match find_clip(&clip) {
			FindClip::One(clip) => Some(clip),
			FindClip::Multiple(clip_a, clip_b) => {
				return Err(format!(
					"Multiple clips matching {} found. Please be more specific.\n\
					> {}\n\
					> {}\n\
					> ...",
					clip,
					clip_a,
					clip_b
				)
				.into())
			}
			FindClip::None => return Err(format!("Clip {} not found", clip).into()),
		},
		None => None,
	};

	let data_lock = ctx.data.read().await;
	let pool = data_lock.clone_expect::<Pool>();

	match clip {
		Some(clip) => {
			Config::set_bot_intro(&pool, &guild_id, &clip)
				.await
				.map_err(|e| {
					error!("Unable to set bot intro: {:?}", e);
					"Unable to set bot intro"
				})?;

			Ok(format!("Set bot intro to {}", clip).into())
		}
		None => {
			let intro = Config::get_bot_intro(&pool, &guild_id).await.map_err(|e| {
				error!("Unable to retrieve bot intro: {:?}", e);
				"Unable to retrieve bot intro"
			})?;

			Ok(format!(
				"Bot intro is {}",
				match intro {
					None => format!("default"),
					Some(intro) => format!("\"{}\"", intro),
				}
			)
			.into())
		}
	}
}
