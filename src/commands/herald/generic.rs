use log::error;

use serenity::client::Context;
use serenity::model::prelude::{GuildId, UserId};

use std::path::Path;

use crate::Pool;
use crate::audio::{find_clip, FindClip};
use crate::configuration::write_config_eprintln;
use crate::configuration::Config;
use crate::util::{GetExpect, Response};

use super::IntroOutroMode::{self, *};

pub async fn intro_outro(
	ctx: &Context,
	mode: IntroOutroMode,
	user_id: UserId,
	clip: Option<String>,
) -> Result<Response, Response> {
	let clip = match clip {
		Some(clip) => match find_clip(&clip) {
			FindClip::One(clip) => Some(clip),
			FindClip::Multiple => {
				return Err(format!(
					"Multiple clips matching {} found. Please be more specific.",
					clip
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
			}.map_err(|e| {
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
			}.map_err(|e| {
				error!("Unable to write user data: {:?}", e);
				"Unable to set intro/outro"
			})?;

			Ok(format!("Set new {} to {}", mode.lowercase(), clip).into())
		}
	}
}

pub async fn introbot(
	ctx: &Context,
	guild_id: Option<GuildId>,
	clip: Option<String>,
) -> Result<Response, Response> {
	let guild_id = guild_id.ok_or("Groups and DMs not supported".to_string())?;

	let clip = match clip {
		Some(clip) => match find_clip(&clip) {
			FindClip::One(clip) => Some(clip),
			FindClip::Multiple => {
				return Err(format!(
					"Multiple clips matching {} found. Please be more specific.",
					clip
				)
				.into())
			}
			FindClip::None => return Err(format!("Clip {} not found", clip).into()),
		},
		None => None,
	};

	let data_lock = ctx.data.read().await;
	let config_arc = data_lock.clone_expect::<Config>();

	match clip {
		Some(clip) => {
			let mut config = config_arc.write().await;

			config.guilds.entry(guild_id).or_default().bot_intro = Some(clip.clone());

			write_config_eprintln(Path::new("config.json"), &*config);

			Ok(format!("Set bot intro to {}", clip).into())
		}
		None => {
			let config = config_arc.read().await;

			let intro = config
				.guilds
				.get(&guild_id)
				.and_then(|c| c.bot_intro.as_ref());

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
