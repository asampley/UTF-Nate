use std::fs::read_to_string;
use std::path::PathBuf;
use std::sync::LazyLock;

use async_trait::async_trait;

use serenity::all::{GuildId, UserId};
use sqlx::{AnyExecutor, Database, Decode, Encode, FromRow, IntoArguments, Type};

use crate::util::Conv;
use crate::RESOURCE_PATH;

use super::{Storage, StorageError};

/// Path to shared directory for database scripts.
pub static DB_PATH: LazyLock<PathBuf> = LazyLock::new(|| RESOURCE_PATH.join("database/"));

#[async_trait]
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
