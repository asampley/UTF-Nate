//! Runtime persistent data such as user and guild settings.
//!
//! Most transient configuration is stored in a database, and can be shared by
//! multiple bot instances. This part of the configuration is fetched using
//! functions on [`Config`].

pub mod database;

use std::fmt;

use async_trait::async_trait;

use serenity::model::id::{GuildId, UserId};

use thiserror::Error;

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

/// Generic trait that can be implemented for the storage.
///
/// It is designed to be shared, thus uses a shared reference. You can always use a Mutex or
/// something if this gets in the way, of course.
#[async_trait]
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

#[cfg(test)]
mod test {
	use sqlx::pool::PoolOptions;

	use super::*;

	/// Get a memory sqlite database for testing
	async fn pool() -> Box<dyn Storage> {
		sqlx::any::install_default_drivers();

		let pool = PoolOptions::<sqlx::Any>::new()
			.max_connections(1)
			.max_lifetime(None)
			.idle_timeout(None)
			.connect("sqlite::memory:")
			.await
			.unwrap();

		pool.first_time_setup().await.unwrap();

		Box::new(pool)
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
