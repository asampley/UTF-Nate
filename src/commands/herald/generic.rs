use serenity::client::Context;
use serenity::model::prelude::{GuildId, UserId};

use std::path::Path;

use crate::audio::{find_clip, FindClip};
use crate::configuration::write_config_eprintln;
use crate::configuration::Config;
use crate::util::GetExpect;

use super::IntroOutroMode::{self, *};

pub async fn intro_outro(
	ctx: &Context,
	mode: IntroOutroMode,
	user_id: UserId,
	clip: Option<String>,
) -> String {
	let clip = match clip {
		Some(clip) => match find_clip(&clip) {
			FindClip::One(clip) => Some(clip),
			FindClip::Multiple => {
				return format!(
					"Multiple clips matching {} found. Please be more specific.",
					clip
				)
			}
			FindClip::None => return format!("Clip {} not found", clip),
		},
		None => None,
	};

	let data_lock = ctx.data.read().await;
	let config_arc = data_lock.clone_expect::<Config>();

	match clip {
		None => {
			let config = config_arc.read().await;

			let clip = match mode {
				Intro => config.intros.get(&user_id),
				Outro => config.outros.get(&user_id),
			};

			format!(
				"User {} is {}",
				mode.lowercase(),
				match clip {
					None => format!("default"),
					Some(clip) => format!("\"{}\"", clip),
				}
			)
		}
		Some(clip) => {
			let mut config = config_arc.write().await;

			match mode {
				Intro => config.intros.insert(user_id, clip.clone()),
				Outro => config.outros.insert(user_id, clip.clone()),
			};

			write_config_eprintln(Path::new("config.json"), &*config);

			format!("Set new {} to {}", mode.lowercase(), clip)
		}
	}
}

pub async fn introbot(ctx: &Context, guild_id: Option<GuildId>, clip: Option<String>) -> String {
	let guild_id = match guild_id {
		Some(guild_id) => guild_id,
		None => return "Groups and DMs not supported".to_string(),
	};

	let clip = match clip {
		Some(clip) => match find_clip(&clip) {
			FindClip::One(clip) => Some(clip),
			FindClip::Multiple => {
				return format!(
					"Multiple clips matching {} found. Please be more specific.",
					clip
				)
			}
			FindClip::None => return format!("Clip {} not found", clip),
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

			format!("Set bot intro to {}", clip)
		}
		None => {
			let config = config_arc.read().await;

			let intro = config
				.guilds
				.get(&guild_id)
				.and_then(|c| c.bot_intro.as_ref());

			format!(
				"Bot intro is {}",
				match intro {
					None => format!("default"),
					Some(intro) => format!("\"{}\"", intro),
				}
			)
		}
	}
}
