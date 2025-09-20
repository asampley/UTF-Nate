//! Read configuration from files and a database.
//!
//! Some configuration is stored on file using [`Config`] and is able to be set
//! per bot instance.
//!
//! Most persistent configuration is stored in a database, and can be shared by
//! multiple bot instances. See [`crate::persistence`].

use serde::{Deserialize, Serialize};

use serenity::gateway::ActivityData;

use std::net::SocketAddr;

/// Configuration struct that holds values from a file, and implements
/// functions to read other values from the database.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
	/// Prefixes that the bot recognizes as beginning a command.
	pub prefixes: Vec<String>,
	pub activity: Option<ActivityConfig>,
	pub http: Option<HttpConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpConfig {
	pub public_url: String,
	pub listen: SocketAddr,
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
