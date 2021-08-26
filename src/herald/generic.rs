use serenity::client::Context;
use serenity::model::prelude::{UserId, GuildId};

use std::path::Path;

use crate::configuration;
use crate::configuration::write_config;
use crate::data::ConfigResource;
use crate::herald::IntroOutroMode::{self, *};
use crate::voice::get_clip;

pub async fn intro_outro(
	ctx: &Context,
	mode: IntroOutroMode,
	user_id: UserId,
	clip: Option<String>,
) -> String {
	let clip = match clip {
		Some(clip) => clip,
		None => return "No clip provided".to_string(),
	};

	match get_clip(&clip) {
		Some(_) => (),
		None => return format!("Invalid clip: {}", clip),
	}

	let mut data_lock = ctx.data.write().await;
	let config_arc = data_lock
		.get_mut::<ConfigResource>()
		.expect("Expected ConfigResource in ShareMap")
		.clone();

	let mut config = config_arc.write().await;

	match mode {
		Intro => config.intros.insert(user_id, clip.clone()),
		Outro => config.outros.insert(user_id, clip.clone()),
	};

	{
		use configuration::Result::*;
		match write_config(Path::new("config.json"), &*config) {
			Ok(()) => (),
			JsonError(reason) => eprintln!("Error writing config file: {:?}", reason),
			IoError(reason) => eprintln!("Error writing config file: {:?}", reason),
		}
	}

	format!("Set new {} to {}", match mode { Intro => "intro", Outro => "outro" }, clip)
}

pub async fn introbot(ctx: &Context, guild_id: Option<GuildId>, clip: Option<String>) -> String {
	let guild_id = match guild_id {
		Some(guild_id) => guild_id,
		None => return "Groups and DMs not supported".to_string(),
	};

	let clip = match clip {
		Some(clip) => clip,
		None => return "No clip provided".to_string(),
	};

	match get_clip(&clip) {
		Some(_) => (),
		None => return format!("Invalid clip: {}", clip),
	}

	let mut data_lock = ctx.data.write().await;
	let config_arc = data_lock
		.get_mut::<ConfigResource>()
		.expect("Expected ConfigResource in ShareMap")
		.clone();

	let mut config = config_arc.write().await;

	config.guilds.entry(guild_id).or_default().bot_intro = Some(clip.clone());

	{
		use configuration::Result::*;
		match write_config(Path::new("config.json"), &*config) {
			Ok(()) => (),
			JsonError(reason) => eprintln!("Error writing config file: {:?}", reason),
			IoError(reason) => eprintln!("Error writing config file: {:?}", reason),
		}
	}

	format!("Set bot intro to {}", clip)
}
