use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};

use serenity::model::prelude::{GuildId, UserId};

/// Token that is used for the web interface.
///
/// Contains details of how the command was called.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Token {
	/// Guild id for the command, which is `None` when there is no guild.
	pub guild_id: Option<GuildId>,

	/// User id that invoked the command. Must always be set.
	pub user_id: UserId,

	/// Expiry timestamp for token
	pub expiry: DateTime<Utc>,
}

impl Token {
	pub fn is_expired(&self) -> bool {
		self.expiry < Utc::now()
	}
}
