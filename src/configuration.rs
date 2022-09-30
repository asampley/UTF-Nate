use once_cell::sync::Lazy;

use serde::{Deserialize, Serialize};
use serenity::model::id::{GuildId, UserId};

use sqlx::PgExecutor as Executor;
use sqlx::{Decode, Encode, Type};

use thiserror::Error;

use std::fmt;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

use crate::RESOURCE_PATH;

const DB_PATH: Lazy<PathBuf> = Lazy::new(|| RESOURCE_PATH.join("database/"));

#[derive(Debug, Error)]
pub enum ConfigError {
	IoError(#[from] std::io::Error),
	DbError(#[from] sqlx::Error),
	NoRowsChanged,
}

impl fmt::Display for ConfigError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Debug::fmt(self, f)
	}
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
	pub prefixes: Vec<String>,
}

impl Config {
	async fn get_by_id<'e, E, I, T>(
		executor: E,
		file_name: &Path,
		id: I,
	) -> Result<Option<T>, ConfigError>
	where
		E: Executor<'e>,
		for<'a> I: Encode<'a, E::Database> + Type<E::Database> + Send,
		for<'a> T: Decode<'a, E::Database> + Type<E::Database> + Send + Unpin,
	{
		Ok(sqlx::query_scalar(&read_to_string(file_name)?)
			.bind(id)
			.fetch_optional(executor)
			.await?)
	}

	async fn set_by_id<'e, E, I, T>(
		executor: E,
		file_name: &Path,
		id: I,
		value: T,
	) -> Result<(), ConfigError>
	where
		E: Executor<'e>,
		for<'a> I: Encode<'a, E::Database> + Type<E::Database> + Send,
		for<'a> T: Encode<'a, E::Database> + Type<E::Database> + Send,
	{
		let res = sqlx::query(&read_to_string(file_name)?)
			.bind(id)
			.bind(value)
			.execute(executor)
			.await?;

		match res.rows_affected() {
			1 => Ok(()),
			_ => Err(ConfigError::NoRowsChanged),
		}
	}

	pub async fn set_intro<'e, E: Executor<'e>>(
		executor: E,
		user_id: &UserId,
		intro: &str,
	) -> Result<(), ConfigError> {
		Config::set_by_id(executor, &DB_PATH.join("set-intro.sql"), user_id.0 as i64, intro).await
	}

	pub async fn get_intro<'e, E: Executor<'e>>(
		executor: E,
		user_id: &UserId,
	) -> Result<Option<String>, ConfigError> {
		Config::get_by_id(executor, &DB_PATH.join("get-intro.sql"), user_id.0 as i64).await
	}

	pub async fn set_outro<'e, E: Executor<'e>>(
		executor: E,
		user_id: &UserId,
		outro: &str,
	) -> Result<(), ConfigError> {
		Config::set_by_id(executor, &DB_PATH.join("set-outro.sql"), user_id.0 as i64, outro).await
	}

	pub async fn get_outro<'e, E: Executor<'e>>(
		executor: E,
		user_id: &UserId,
	) -> Result<Option<String>, ConfigError> {
		Config::get_by_id(executor, &DB_PATH.join("get-outro.sql"), user_id.0 as i64).await
	}

	pub async fn set_bot_intro<'e, E: Executor<'e>>(
		executor: E,
		guild_id: &GuildId,
		intro: &str,
	) -> Result<(), ConfigError> {
		Config::set_by_id(
			executor,
			&DB_PATH.join("set-bot-intro.sql"),
			guild_id.0 as i64,
			intro,
		)
		.await
	}

	pub async fn get_bot_intro<'e, E: Executor<'e>>(
		executor: E,
		guild_id: &GuildId,
	) -> Result<Option<String>, ConfigError> {
		Config::get_by_id(executor, &DB_PATH.join("get-bot-intro.sql"), guild_id.0 as i64).await
	}

	pub async fn set_volume_play<'e, E: Executor<'e>>(
		executor: E,
		guild_id: &GuildId,
		volume: f32,
	) -> Result<(), ConfigError> {
		Config::set_by_id(
			executor,
			&DB_PATH.join("set-volume-play.sql"),
			guild_id.0 as i64,
			volume,
		)
		.await
	}

	pub async fn get_volume_play<'e, E: Executor<'e>>(
		executor: E,
		guild_id: &GuildId,
	) -> Result<Option<f32>, ConfigError> {
		Config::get_by_id(executor, &DB_PATH.join("get-volume-play.sql"), guild_id.0 as i64).await
	}

	pub async fn set_volume_clip<'e, E: Executor<'e>>(
		executor: E,
		guild_id: &GuildId,
		volume: f32,
	) -> Result<(), ConfigError> {
		Config::set_by_id(
			executor,
			&DB_PATH.join("set-volume-clip.sql"),
			guild_id.0 as i64,
			volume,
		)
		.await
	}

	pub async fn get_volume_clip<'e, E: Executor<'e>>(
		executor: E,
		guild_id: &GuildId,
	) -> Result<Option<f32>, ConfigError> {
		Config::get_by_id(executor, &DB_PATH.join("get-volume-clip.sql"), guild_id.0 as i64).await
	}
}
