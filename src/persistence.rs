//! Runtime persistent data such as user and guild settings.
//!
//! Most transient configuration is stored in a database, and can be shared by
//! multiple bot instances. This part of the configuration is fetched using
//! functions on [`Config`].

use std::path::PathBuf;
use std::sync::LazyLock;
use std::{fmt, fs::read_to_string};

use serenity::model::id::{GuildId, UserId};

use sqlx::{AnyExecutor, Database, Decode, Encode, FromRow, IntoArguments, Type};
use thiserror::Error;

use crate::util::Conv;
use crate::RESOURCE_PATH;

/// Path to shared directory for database scripts.
pub static DB_PATH: LazyLock<PathBuf> = LazyLock::new(|| RESOURCE_PATH.join("database/"));

/// Generic trait that can be implemented for the storage.
///
/// It is designed to be shared, thus uses a shared reference. You can always use a Mutex or
/// something if this gets in the way, of course.
pub trait Storage {
	/// Setup the storage the *very first* time it is initialised. This is meant to persist, so
	/// shouldn't be called every time you want to use the storage. It is expected to, for example,
	/// create tables in a database or initialize a file.
	async fn first_time_setup(&self) -> Result<(), StorageError>;

	/// Get the intro for a user. This will return exactly the value set using [`set_intro`].
	async fn get_intro(&self, user_id: UserId) -> Result<Option<String>, StorageError>;

	/// Set the intro for a user. This should be the exact file name of the intro. This can later
	/// be retrieved using [`get_intro`].
	async fn set_intro(&self, user_id: UserId, intro: &str) -> Result<(), StorageError>;

	/// Get the outro for a user. This will return exactly the value set using [`set_outro`].
	async fn get_outro(&self, user_id: UserId) -> Result<Option<String>, StorageError>;

	/// Set the outro for a user. This should be the exact file name of the
	/// outro. This can later be retrieved using [`get_outro`].
	async fn set_outro(&self, user_id: UserId, outro: &str) -> Result<(), StorageError>;

	/// Get the intro for a bot. This will return exactly the value set using [`set_intro`].
	async fn get_bot_intro(&self, guild_id: GuildId) -> Result<Option<String>, StorageError>;

	/// Set the intro for a the bot. This should be the exact file name of the intro. This can
	/// later be retrieved using [`get_bot_intro`].
	async fn set_bot_intro(&self, guild_id: GuildId, intro: &str) -> Result<(), StorageError>;

	/// Get the volume that should be used when a bot clip is played. Returns whatever volume was
	/// set using [`set_volume_clip`].
	///
	/// This is different from audio sources that are queued (which should use
	/// [`get_volume_play`]).
	async fn get_volume_clip(&self, guild_id: GuildId) -> Result<Option<f32>, StorageError>;

	/// Set the volume that should be used when a bot clip is played, which can later be retrieved
	/// with [`get_volume_clip`].
	///
	/// This is different from audio sources that are queued (which should use
	/// [`set_volume_play`]).
	async fn set_volume_clip(&self, guild_id: GuildId, volume: f32) -> Result<(), StorageError>;

	/// Get the volume that should be used when an audio source is queued. Returns whatever volume
	/// was set using [`set_volume_play`].
	///
	/// This is different from audio clips that the bot stores (which should use
	/// [`get_volume_clip`]).
	async fn get_volume_play(&self, guild_id: GuildId) -> Result<Option<f32>, StorageError>;

	/// Set the volume that should be used when an audio source is queued, which can later be
	/// retrieved with [`get_volume_play`].
	///
	/// This is different from audio clips that the bot stores (which should use
	/// [`set_volume_clip`]).
	async fn set_volume_play(&self, guild_id: GuildId, volume: f32) -> Result<(), StorageError>;
}

#[derive(Debug, Error)]
pub enum StorageError {
	IoError(#[from] std::io::Error),
	DbError(#[from] sqlx::Error),
	NoRowsChanged,
}

impl fmt::Display for StorageError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Debug::fmt(self, f)
	}
}

impl Storage for sqlx::Pool<sqlx::Any> {
	async fn first_time_setup(&self) -> Result<(), StorageError> {
		let sql: &str = &read_query("create-tables.sql")?;
		sqlx::raw_sql(sql).execute(self).await?;
		Ok(())
	}

	async fn get_intro(&self, user_id: UserId) -> Result<Option<String>, StorageError> {
		get_by_id(self, &read_query("get-intro.sql")?, user_id.conv::<i64>()).await
	}

	async fn set_intro(&self, user_id: UserId, intro: &str) -> Result<(), StorageError> {
		set_by_id(
			self,
			&read_query("set-intro.sql")?,
			user_id.conv::<i64>(),
			intro,
		)
		.await
	}

	async fn get_outro(&self, user_id: UserId) -> Result<Option<String>, StorageError> {
		get_by_id(self, &read_query("get-outro.sql")?, user_id.conv::<i64>()).await
	}

	async fn set_outro(&self, user_id: UserId, outro: &str) -> Result<(), StorageError> {
		set_by_id(
			self,
			&read_query("set-outro.sql")?,
			user_id.conv::<i64>(),
			outro,
		)
		.await
	}

	async fn get_bot_intro(&self, guild_id: GuildId) -> Result<Option<String>, StorageError> {
		get_by_id(
			self,
			&read_query("get-bot-intro.sql")?,
			guild_id.conv::<i64>(),
		)
		.await
	}

	async fn set_bot_intro(&self, guild_id: GuildId, intro: &str) -> Result<(), StorageError> {
		set_by_id(
			self,
			&read_query("set-bot-intro.sql")?,
			guild_id.conv::<i64>(),
			intro,
		)
		.await
	}

	async fn get_volume_clip(&self, guild_id: GuildId) -> Result<Option<f32>, StorageError> {
		let mut conn = self.acquire().await?;

		Ok(match conn.backend_name() {
			"SQLite" => get_by_id::<_, _, f64>(
				conn.as_mut(),
				&read_query("get-volume-clip.sql")?,
				guild_id.conv::<i64>(),
			)
			.await?
			.map(|v| v as f32),
			_ => {
				get_by_id::<_, _, f32>(
					conn.as_mut(),
					&read_query("get-volume-clip.sql")?,
					guild_id.conv::<i64>(),
				)
				.await?
			}
		})
	}

	async fn set_volume_clip(&self, guild_id: GuildId, volume: f32) -> Result<(), StorageError> {
		set_by_id(
			self,
			&read_query("set-volume-clip.sql")?,
			guild_id.conv::<i64>(),
			volume,
		)
		.await
	}

	async fn get_volume_play(&self, guild_id: GuildId) -> Result<Option<f32>, StorageError> {
		let mut conn = self.acquire().await?;

		Ok(match conn.backend_name() {
			"SQLite" => get_by_id::<_, _, f64>(
				conn.as_mut(),
				&read_query("get-volume-play.sql")?,
				guild_id.conv::<i64>(),
			)
			.await?
			.map(|v| v as f32),
			_ => {
				get_by_id::<_, _, f32>(
					conn.as_mut(),
					&read_query("get-volume-play.sql")?,
					guild_id.conv::<i64>(),
				)
				.await?
			}
		})
	}

	async fn set_volume_play(&self, guild_id: GuildId, volume: f32) -> Result<(), StorageError> {
		set_by_id(
			self,
			&read_query("set-volume-play.sql")?,
			guild_id.conv::<i64>(),
			volume,
		)
		.await
	}
}

/// Generic implementation to get a single value by using an id.
///
/// `id` is bound into the first variable passed into the database script
/// at `file_name`.
async fn get_by_id<'e, 'q, E, I, T>(
	executor: E,
	sql: &'q str,
	id: I,
) -> Result<Option<T>, StorageError>
where
	E: AnyExecutor<'e>,
	<E::Database as Database>::Arguments<'q>: IntoArguments<'q, E::Database>,
	I: Encode<'q, E::Database> + Type<E::Database> + Send + 'q,
	T: Decode<'q, E::Database> + Type<E::Database> + Send + Unpin + 'q,
	for<'r> (T,): FromRow<'r, <E::Database as Database>::Row>,
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
) -> Result<(), StorageError>
where
	E: AnyExecutor<'e>,
	<E::Database as Database>::Arguments<'q>: IntoArguments<'q, E::Database>,
	I: Encode<'q, E::Database> + Type<E::Database> + Send + 'q,
	T: Encode<'q, E::Database> + Type<E::Database> + Send + 'q,
{
	sqlx::query(sql)
		.bind(id)
		.bind(value)
		.execute(executor)
		.await
		.map_err(Into::into)
		.and_then(|query| match query.rows_affected() {
			0 => Err(StorageError::NoRowsChanged),
			_ => Ok(()),
		})
}

fn read_query(name: &str) -> Result<String, StorageError> {
	read_to_string(DB_PATH.join(name)).map_err(Into::into)
}

#[cfg(test)]
mod test {
	use sqlx::pool::PoolOptions;

	use super::*;

	/// Get a memory sqlite database for testing
	async fn pool() -> sqlx::Pool<sqlx::Any> {
		sqlx::any::install_default_drivers();

		let pool = PoolOptions::<sqlx::Any>::new()
			.max_connections(1)
			.max_lifetime(None)
			.idle_timeout(None)
			.connect("sqlite::memory:")
			.await
			.unwrap();

		pool.first_time_setup().await.unwrap();

		pool
	}

	const ERROR_GET: &str = "Error while getting";
	const ERROR_SET: &str = "Error while setting";

	#[tokio::test]
	async fn get_intro_unset() {
		let db = pool().await;

		let get = db.get_intro(UserId::new(1)).await.expect(ERROR_GET);

		assert_eq!(get, None)
	}

	#[tokio::test]
	async fn get_outro_unset() {
		let db = pool().await;

		let get = db.get_outro(UserId::new(1)).await.expect(ERROR_GET);

		assert_eq!(get, None)
	}

	#[tokio::test]
	async fn get_bot_intro_unset() {
		let db = pool().await;

		let get = db.get_bot_intro(GuildId::new(1)).await.expect(ERROR_GET);

		assert_eq!(get, None)
	}

	#[tokio::test]
	async fn get_volume_clip_unset() {
		let db = pool().await;

		let get = db.get_volume_clip(GuildId::new(1)).await.expect(ERROR_GET);

		assert_eq!(get, None)
	}

	#[tokio::test]
	async fn get_volume_play_unset() {
		let db = pool().await;

		let get = db.get_volume_play(GuildId::new(1)).await.expect(ERROR_GET);

		assert_eq!(get, None)
	}

	#[tokio::test]
	async fn set_get_intro() {
		let db = pool().await;

		let set = "test";

		db.set_intro(UserId::new(1), set).await.expect(ERROR_SET);

		let get = db.get_intro(UserId::new(1)).await.expect(ERROR_GET);

		assert_eq!(get.as_deref(), Some(set));
	}

	#[tokio::test]
	async fn set_get_outro() {
		let db = pool().await;

		let set = "test";

		db.set_outro(UserId::new(1), set).await.expect(ERROR_SET);

		let get = db.get_outro(UserId::new(1)).await.expect(ERROR_GET);

		assert_eq!(get.as_deref(), Some(set));
	}

	#[tokio::test]
	async fn set_get_bot_intro() {
		let db = pool().await;

		let set = "test";

		db.set_bot_intro(GuildId::new(1), set)
			.await
			.expect(ERROR_SET);

		let get = db.get_bot_intro(GuildId::new(1)).await.expect(ERROR_GET);

		assert_eq!(get.as_deref(), Some(set));
	}

	#[tokio::test]
	async fn set_get_volume_clip() {
		let db = pool().await;

		let set = 0.1234;

		db.set_volume_clip(GuildId::new(1), set)
			.await
			.expect(ERROR_SET);

		let get = db.get_volume_clip(GuildId::new(1)).await.expect(ERROR_GET);

		assert_eq!(get, Some(set));
	}

	#[tokio::test]
	async fn set_get_volume_play() {
		let db = pool().await;

		let set = 0.1234;

		db.set_volume_play(GuildId::new(1), set)
			.await
			.expect(ERROR_SET);

		let get = db.get_volume_play(GuildId::new(1)).await.expect(ERROR_GET);

		assert_eq!(get, Some(set));
	}
}
