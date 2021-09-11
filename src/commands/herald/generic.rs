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
			FindClip::One(clip) => clip,
			FindClip::Multiple => {
				return format!(
					"Multiple clips matching {} found. Please be more specific.",
					clip
				)
			}
			FindClip::None => format!("Clip {} not found", clip),
		},
		None => return "No clip provided".to_string(),
	};

	let data_lock = ctx.data.read().await;
	let config_arc = data_lock.clone_expect::<Config>();

	let mut config = config_arc.write().await;

	match mode {
		Intro => config.intros.insert(user_id, clip.clone()),
		Outro => config.outros.insert(user_id, clip.clone()),
	};

	write_config_eprintln(Path::new("config.json"), &*config);

	format!(
		"Set new {} to {}",
		match mode {
			Intro => "intro",
			Outro => "outro",
		},
		clip
	)
}

pub async fn introbot(ctx: &Context, guild_id: Option<GuildId>, clip: Option<String>) -> String {
	let guild_id = match guild_id {
		Some(guild_id) => guild_id,
		None => return "Groups and DMs not supported".to_string(),
	};

	let clip = match clip {
		Some(clip) => match find_clip(&clip) {
			FindClip::One(clip) => clip,
			FindClip::Multiple => {
				return format!(
					"Multiple clips matching {} found. Please be more specific.",
					clip
				)
			}
			FindClip::None => format!("Clip {} not found", clip),
		},
		None => return "No clip provided".to_string(),
	};

	let data_lock = ctx.data.read().await;
	let config_arc = data_lock.clone_expect::<Config>();

	let mut config = config_arc.write().await;

	config.guilds.entry(guild_id).or_default().bot_intro = Some(clip.clone());

	write_config_eprintln(Path::new("config.json"), &*config);

	format!("Set bot intro to {}", clip)
}
