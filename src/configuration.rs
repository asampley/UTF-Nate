use log::error;

use serde::{Deserialize, Serialize};
use serenity::model::id::{GuildId, UserId};
use serenity::prelude::TypeMapKey;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

use crate::data::ArcRw;
use crate::util::JsonFileError;

#[derive(Default, Serialize, Deserialize)]
pub struct Config {
	pub intros: HashMap<UserId, String>,
	pub outros: HashMap<UserId, String>,
	pub guilds: HashMap<GuildId, GuildConfig>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct GuildConfig {
	pub bot_intro: Option<String>,
	pub volume_clip: Option<f32>,
	pub volume_play: Option<f32>,
}

impl TypeMapKey for Config {
	type Value = ArcRw<Config>;
}

pub fn write_config(path: &Path, config: &Config) -> Result<(), JsonFileError> {
	let file = File::create(path)?;

	Ok(serde_json::to_writer_pretty(file, config)?)
}

pub fn write_config_eprintln(path: &Path, config: &Config) {
	use JsonFileError::*;

	match write_config(path, &*config) {
		Ok(()) => (),
		Err(e) => match e {
			JsonError(reason) => error!("Error writing config file: {:?}", reason),
			IoError(reason) => error!("Error writing config file: {:?}", reason),
		},
	}
}

pub fn read_config(path: &Path) -> Result<Config, JsonFileError> {
	let file = File::open(path)?;

	Ok(serde_json::from_reader(file)?)
}
