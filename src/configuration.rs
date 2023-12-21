//! Read configuration from files and a database.
//!
//! Some configuration is stored on file using [`Config`] and is able to be set
//! per bot instance.
//!
//! Most transient configuration is stored in a database, and can be shared by
//! multiple bot instances. This part of the configuration is fetched using
//! functions on [`Config`].

use once_cell::sync::Lazy;

use serde::{Deserialize, Serialize};

use serenity::gateway::ActivityData;
use serenity::model::id::{GuildId, UserId};

use sqlx::AnyExecutor as Executor;
use sqlx::{Decode, Encode, Type};

use tap::Conv;
use thiserror::Error;

use std::fmt;
use std::fs::read_to_string;
use std::net::SocketAddr;
use std::path::PathBuf;

use crate::RESOURCE_PATH;

/// Path to shared directory for database scripts.
pub static DB_PATH: Lazy<PathBuf> = Lazy::new(|| RESOURCE_PATH.join("database/"));

/// Configuration for an error that can come from trying to get or set config
/// values.
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

/// Configuration struct that holds values from a file, and implements
/// functions to read other values from the database.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
	/// Prefixes that the bot recognizes as beginning a command.
	pub prefixes: Vec<String>,
	pub activity: Option<ActivityConfig>,
	pub http: Option<SocketAddr>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ActivityConfig {
	Competing { name: String },
	Listening { name: String },
	Playing { name: String },
	Streaming { name: String, url: String },
	Watching { name: String },
}

impl TryFrom<&ActivityConfig> for ActivityData {
	type Error = serenity::Error;

	fn try_from(value: &ActivityConfig) -> Result<Self, Self::Error> {
		Ok(match value {
			ActivityConfig::Competing { name } => ActivityData::competing(name),
			ActivityConfig::Listening { name } => ActivityData::listening(name),
			ActivityConfig::Playing { name } => ActivityData::playing(name),
			ActivityConfig::Streaming { name, url } => ActivityData::streaming(name, url)?,
			ActivityConfig::Watching { name } => ActivityData::watching(name),
		})
	}
}

impl Config {
	fn read_query(name: &str) -> Result<String, ConfigError> {
		read_to_string(DB_PATH.join(name)).map_err(Into::into)
	}

	/// Generic implementation to get a single value by using an id.
	///
	/// `id` is bound into the first variable passed into the database script
	/// at `file_name`.
	async fn get_by_id<'e, E, I, T>(executor: E, sql: &str, id: I) -> Result<Option<T>, ConfigError>
	where
		E: Executor<'e>,
		I: for<'a> Encode<'a, E::Database> + Type<E::Database> + Send,
		T: for<'a> Decode<'a, E::Database> + Type<E::Database> + Send + Unpin,
	{
		Ok(sqlx::query_scalar(sql)
			.bind(id)
			.fetch_optional(executor)
			.await?)
	}

	/// Generic implementation to set a single value by using an id.
	///
	/// `id` is bound into the first variable passed into the database script
	/// at `file_name`. `value` is bound to the second variable.
	async fn set_by_id<'e, 'q, E, I, T>(
		executor: E,
		sql: &'q str,
		id: I,
		value: T,
	) -> Result<(), ConfigError>
	where
		E: Executor<'e>,
		I: Encode<'q, E::Database> + Type<E::Database> + Send + 'q,
		T: Encode<'q, E::Database> + Type<E::Database> + Send + 'q,
	{
		let res = sqlx::query(sql)
			.bind(id)
			.bind(value)
			.execute(executor)
			.await?;

		match res.rows_affected() {
			1 => Ok(()),
			_ => Err(ConfigError::NoRowsChanged),
		}
	}

	pub async fn setup_db<'e, E: Executor<'e>>(executor: E) -> Result<(), ConfigError> {
		sqlx::query(&Self::read_query("create-tables.sql")?)
			.execute(executor)
			.await?;

		Ok(())
	}

	/// Set the intro for a user. This should be the exact file name of the
	/// intro. This can later be retrieved using [`Self::get_intro`].
	pub async fn set_intro<'e, E: Executor<'e>>(
		executor: E,
		user_id: UserId,
		intro: &str,
	) -> Result<(), ConfigError> {
		Self::set_by_id(
			executor,
			&Self::read_query("set-intro.sql")?,
			user_id.conv::<i64>(),
			intro,
		)
		.await
	}

	/// Get the intro for a user. This will return exactly the value set using
	/// [`Self::set_intro`].
	pub async fn get_intro<'e, E: Executor<'e>>(
		executor: E,
		user_id: UserId,
	) -> Result<Option<String>, ConfigError> {
		Self::get_by_id(
			executor,
			&Self::read_query("get-intro.sql")?,
			user_id.conv::<i64>(),
		)
		.await
	}

	/// Set the outro for a user. This should be the exact file name of the
	/// outro. This can later be retrieved using [`Self::get_outro`].
	pub async fn set_outro<'e, E: Executor<'e>>(
		executor: E,
		user_id: UserId,
		outro: &str,
	) -> Result<(), ConfigError> {
		Self::set_by_id(
			executor,
			&Self::read_query("set-outro.sql")?,
			user_id.conv::<i64>(),
			outro,
		)
		.await
	}

	/// Get the outro for a user. This will return exactly the value set using
	/// [`Self::set_outro`].
	pub async fn get_outro<'e, E: Executor<'e>>(
		executor: E,
		user_id: UserId,
	) -> Result<Option<String>, ConfigError> {
		Self::get_by_id(
			executor,
			&Self::read_query("get-outro.sql")?,
			user_id.conv::<i64>(),
		)
		.await
	}

	/// Set the intro for a the bot. This should be the exact file name of the
	/// intro. This can later be retrieved using [`Self::get_bot_intro`].
	pub async fn set_bot_intro<'e, E: Executor<'e>>(
		executor: E,
		guild_id: GuildId,
		intro: &str,
	) -> Result<(), ConfigError> {
		Self::set_by_id(
			executor,
			&Self::read_query("set-bot-intro.sql")?,
			guild_id.conv::<i64>(),
			intro,
		)
		.await
	}

	/// Get the intro for a bot. This will return exactly the value set using
	/// [`Self::set_intro`].
	pub async fn get_bot_intro<'e, E: Executor<'e>>(
		executor: E,
		guild_id: GuildId,
	) -> Result<Option<String>, ConfigError> {
		Self::get_by_id(
			executor,
			&Self::read_query("get-bot-intro.sql")?,
			guild_id.conv::<i64>(),
		)
		.await
	}

	/// Set the volume that should be used when an audio source is queued,
	/// which can later be retrieved with [`Self::get_volume_play`].
	///
	/// This is different from audio clips that the bot stores (which should use
	/// [`Self::set_volume_clip`]).
	pub async fn set_volume_play<'e, E: Executor<'e>>(
		executor: E,
		guild_id: GuildId,
		volume: f32,
	) -> Result<(), ConfigError> {
		Self::set_by_id(
			executor,
			&Self::read_query("set-volume-play.sql")?,
			guild_id.conv::<i64>(),
			volume,
		)
		.await
	}

	/// Get the volume that should be used when an audio source is queued.
	/// Returns whatever volume was set using [`Self::set_volume_play`].
	///
	/// This is different from audio clips that the bot stores (which should use
	/// [`Self::get_volume_clip`]).
	pub async fn get_volume_play<'e, E: Executor<'e>>(
		executor: E,
		guild_id: GuildId,
	) -> Result<Option<f32>, ConfigError> {
		Self::get_by_id(
			executor,
			&Self::read_query("get-volume-play.sql")?,
			guild_id.conv::<i64>(),
		)
		.await
	}

	/// Set the volume that should be used when a bot clip is played, which can
	/// later be retrieved with [`Self::get_volume_clip`].
	///
	/// This is different from audio sources that are queued (which should use
	/// [`Self::set_volume_play`]).
	pub async fn set_volume_clip<'e, E: Executor<'e>>(
		executor: E,
		guild_id: GuildId,
		volume: f32,
	) -> Result<(), ConfigError> {
		Self::set_by_id(
			executor,
			&Self::read_query("set-volume-clip.sql")?,
			guild_id.conv::<i64>(),
			volume,
		)
		.await
	}

	/// Get the volume that should be used when a bot clip is played. Returns
	/// whatever volume was set using [`Self::set_volume_clip`].
	///
	/// This is different from audio sources that are queued (which should use
	/// [`Self::get_volume_play`]).
	pub async fn get_volume_clip<'e, E: Executor<'e>>(
		executor: E,
		guild_id: GuildId,
	) -> Result<Option<f32>, ConfigError> {
		Self::get_by_id(
			executor,
			&Self::read_query("get-volume-clip.sql")?,
			guild_id.conv::<i64>(),
		)
		.await
	}
}

#[cfg(test)]
mod test {
	use sqlx::pool::PoolOptions;

	use super::*;

	/// Get a memory sqlite database for testing
	async fn pool() -> sqlx::AnyPool {
		sqlx::any::install_default_drivers();

		let pool = PoolOptions::new()
			.max_connections(1)
			.max_lifetime(None)
			.idle_timeout(None)
			.connect("sqlite::memory:")
			.await
			.unwrap();

		Config::setup_db(&pool).await.unwrap();

		pool
	}

	const ERROR_GET: &str = "Error while getting";
	const ERROR_SET: &str = "Error while setting";

	#[tokio::test]
	async fn get_intro_unset() {
		let db = pool().await;

		let get = Config::get_intro(&db, UserId::new(1))
			.await
			.expect(ERROR_GET);

		assert_eq!(get, None)
	}

	#[tokio::test]
	async fn get_outro_unset() {
		let db = pool().await;

		let get = Config::get_outro(&db, UserId::new(1))
			.await
			.expect(ERROR_GET);

		assert_eq!(get, None)
	}

	#[tokio::test]
	async fn get_bot_intro_unset() {
		let db = pool().await;

		let get = Config::get_bot_intro(&db, GuildId::new(1))
			.await
			.expect(ERROR_GET);

		assert_eq!(get, None)
	}

	#[tokio::test]
	async fn get_volume_clip_unset() {
		let db = pool().await;

		let get = Config::get_volume_clip(&db, GuildId::new(1))
			.await
			.expect(ERROR_GET);

		assert_eq!(get, None)
	}

	#[tokio::test]
	async fn get_volume_play_unset() {
		let db = pool().await;

		let get = Config::get_volume_play(&db, GuildId::new(1))
			.await
			.expect(ERROR_GET);

		assert_eq!(get, None)
	}

	#[tokio::test]
	async fn set_get_intro() {
		let db = pool().await;

		let set = "test";

		Config::set_intro(&db, UserId::new(1), set)
			.await
			.expect(ERROR_SET);

		let get = Config::get_intro(&db, UserId::new(1))
			.await
			.expect(ERROR_GET);

		assert_eq!(get.as_deref(), Some(set));
	}

	#[tokio::test]
	async fn set_get_outro() {
		let db = pool().await;

		let set = "test";

		Config::set_outro(&db, UserId::new(1), set)
			.await
			.expect(ERROR_SET);

		let get = Config::get_outro(&db, UserId::new(1))
			.await
			.expect(ERROR_GET);

		assert_eq!(get.as_deref(), Some(set));
	}

	#[tokio::test]
	async fn set_get_bot_intro() {
		let db = pool().await;

		let set = "test";

		Config::set_bot_intro(&db, GuildId::new(1), set)
			.await
			.expect(ERROR_SET);

		let get = Config::get_bot_intro(&db, GuildId::new(1))
			.await
			.expect(ERROR_GET);

		assert_eq!(get.as_deref(), Some(set));
	}

	#[tokio::test]
	async fn set_get_volume_clip() {
		let db = pool().await;

		let set = 0.1234;

		Config::set_volume_clip(&db, GuildId::new(1), set)
			.await
			.expect(ERROR_SET);

		let get = Config::get_volume_clip(&db, GuildId::new(1))
			.await
			.expect(ERROR_GET);

		assert_eq!(get, Some(set));
	}

	#[tokio::test]
	async fn set_get_volume_play() {
		let db = pool().await;

		let set = 0.1234;

		Config::set_volume_play(&db, GuildId::new(1), set)
			.await
			.expect(ERROR_SET);

		let get = Config::get_volume_play(&db, GuildId::new(1))
			.await
			.expect(ERROR_GET);

		assert_eq!(get, Some(set));
	}
}
