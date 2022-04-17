use serde::{Deserialize, Serialize};
use serenity::model::id::{GuildId, UserId};

use sqlx::PgExecutor as Executor;
use sqlx::{Encode, Decode, Type};

use std::fs::{File, read_to_string};
use std::path::Path;

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
	pub prefixes: Vec<String>
}

impl Config {
	async fn get_by_id<'e, E, I, T>(
		executor: E,
		file_name: &str,
		id: I,
	) -> Result<Option<T>, ConfigError> where
		E: Executor<'e>,
		for<'a> I: Encode<'a, E::Database> + Type<E::Database> + Send,
		for<'a> T: Decode<'a, E::Database> + Type<E::Database> + Send + Unpin,
	{
		Ok(sqlx::query_scalar(&read_to_string(file_name)?)
			.bind(id)
			.fetch_optional(executor).await?)
	}

	async fn set_by_id<'e, E, I, T>(
		executor: E,
		file_name: &str,
		id: I,
		value: T,
	) -> Result<(), ConfigError> where
		E: Executor<'e>,
		for<'a> I: Encode<'a, E::Database> + Type<E::Database> + Send,
		for<'a> T: Encode<'a, E::Database> + Type<E::Database> + Send,
	{
		let res = sqlx::query(&read_to_string(file_name)?)
			.bind(id)
			.bind(value)
			.execute(executor).await?;

		match res.rows_affected() {
			1 => Ok(()),
			_ => Err(ConfigError::NoRowsChanged)
		}
	}

	pub async fn set_intro<'e, E: Executor<'e>>(
		executor: E,
		user_id: &UserId,
		intro: &str,
	) -> Result<(), ConfigError> {
		Config::set_by_id(executor, "database/set-intro.sql", user_id.0 as i64, intro).await
	}

	pub async fn get_intro<'e, E: Executor<'e>>(
		executor: E,
		user_id: &UserId,
	) -> Result<Option<String>, ConfigError> {
		Config::get_by_id(executor, "database/get-intro.sql", user_id.0 as i64).await
	}

	pub async fn set_outro<'e, E: Executor<'e>>(
		executor: E,
		user_id: &UserId,
		outro: &str,
	) -> Result<(), ConfigError> {
		Config::set_by_id(executor, "database/set-outro.sql", user_id.0 as i64, outro).await
	}

	pub async fn get_outro<'e, E: Executor<'e>>(
		executor: E,
		user_id: &UserId,
	) -> Result<Option<String>, ConfigError> {
		Config::get_by_id(executor, "database/get-outro.sql", user_id.0 as i64).await
	}

	pub async fn set_bot_intro<'e, E: Executor<'e>>(
		executor: E,
		guild_id: &GuildId,
		intro: &str,
	) -> Result<(), ConfigError> {
		Config::set_by_id(executor, "database/set-bot-intro.sql", guild_id.0 as i64, intro).await
	}

	pub async fn get_bot_intro<'e, E: Executor<'e>>(
		executor: E,
		guild_id: &GuildId,
	) -> Result<Option<String>, ConfigError> {
		Config::get_by_id(executor, "database/get-bot-intro.sql", guild_id.0 as i64).await
	}

	pub async fn set_volume_play<'e, E: Executor<'e>>(
		executor: E,
		guild_id: &GuildId,
		volume: f32,
	) -> Result<(), ConfigError> {
		Config::set_by_id(executor, "database/set-volume-play.sql", guild_id.0 as i64, volume).await
	}

	pub async fn get_volume_play<'e, E: Executor<'e>>(
		executor: E,
		guild_id: &GuildId,
	) -> Result<Option<f32>, ConfigError> {
		Config::get_by_id(executor, "database/get-volume-play.sql", guild_id.0 as i64).await
	}

	pub async fn set_volume_clip<'e, E: Executor<'e>>(
		executor: E,
		guild_id: &GuildId,
		volume: f32,
	) -> Result<(), ConfigError> {
		Config::set_by_id(executor, "database/set-volume-clip.sql", guild_id.0 as i64, volume).await
	}

	pub async fn get_volume_clip<'e, E: Executor<'e>>(
		executor: E,
		guild_id: &GuildId,
	) -> Result<Option<f32>, ConfigError> {
		Config::get_by_id(executor, "database/get-volume-clip.sql", guild_id.0 as i64).await
	}
}

pub fn read_config(path: &Path) -> Result<Config, JsonFileError> {
	let file = File::open(path)?;

	Ok(serde_json::from_reader(file)?)
}
