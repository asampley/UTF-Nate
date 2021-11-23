use fxhash::FxBuildHasher as BuildHasher;

use log::error;

use serde::{Deserialize, Serialize};
use serenity::model::id::{GuildId, UserId};
use serenity::prelude::TypeMapKey;

use sqlx::PgExecutor as Executor;

use std::collections::HashMap;
use std::fs::{File, read_to_string};
use std::path::Path;

use crate::data::ArcRw;
use crate::util::JsonFileError;

#[derive(Debug)]
pub enum ConfigError {
	IoError(std::io::Error),
	DbError(sqlx::Error),
	NoRowsChanged,
}

impl From<sqlx::Error> for ConfigError {
	fn from(e: sqlx::Error) -> Self {
		Self::DbError(e)
	}
}

impl From<std::io::Error> for ConfigError {
	fn from(e: std::io::Error) -> Self {
		Self::IoError(e)
	}
}

#[derive(Default, Serialize, Deserialize)]
pub struct Config {
	pub intros: HashMap<UserId, String, BuildHasher>,
	pub outros: HashMap<UserId, String, BuildHasher>,
	pub guilds: HashMap<GuildId, GuildConfig, BuildHasher>,
}

impl Config {
	pub async fn set_intro<'e, E: Executor<'e>>(
		executor: E,
		user_id: &UserId,
		intro: &str,
	) -> Result<(), ConfigError> {
		let res = sqlx::query(&read_to_string("database/set-intro.sql")?)
			.bind(user_id.0 as i64)
			.bind(intro)
			.execute(executor).await?;

		if res.rows_affected() != 1 {
			Err(ConfigError::NoRowsChanged)
		} else {
			Ok(())
		}
	}

	pub async fn get_intro<'e, E: Executor<'e>>(executor: E, user_id: &UserId) -> Result<Option<String>, ConfigError> {
		Ok(sqlx::query_scalar(&read_to_string("database/get-intro.sql")?)
			.bind(user_id.0 as i64)
			.fetch_optional(executor).await?)
	}

	pub async fn set_outro<'e, E: Executor<'e>>(
		executor: E,
		user_id: &UserId,
		intro: &str,
	) -> Result<(), ConfigError> {
		let res = sqlx::query(&read_to_string("database/set-outro.sql")?)
			.bind(user_id.0 as i64)
			.bind(intro)
			.execute(executor).await?;

		if res.rows_affected() != 1 {
			Err(ConfigError::NoRowsChanged)
		} else {
			Ok(())
		}
	}

	pub async fn get_outro<'e, E: Executor<'e>>(executor: E, user_id: &UserId) -> Result<Option<String>, ConfigError> {
		Ok(sqlx::query_scalar("select outro from user_config where user_id = $1")
			.bind(user_id.0 as i64)
			.fetch_optional(executor).await?)
	}
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
