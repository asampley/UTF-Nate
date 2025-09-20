use axum_extra::extract::CookieJar;

use chrono::{Months, Utc};

use serde::de::Error;

use crate::commands::{BotState, Source};
use crate::encrypt::Encrypted;
use crate::http::Token;
use crate::util::{GetExpect, Response};
use crate::{AeadKey, CONFIG};

#[cfg(feature = "http-interface")]
pub mod http;
pub mod poise;

pub const fn token_help() -> &'static str {
	include_str!("help/token.md")
}

impl TryFrom<&CookieJar> for Encrypted {
	type Error = serde_urlencoded::de::Error;

	fn try_from(value: &CookieJar) -> Result<Self, Self::Error> {
		serde_urlencoded::from_str(
			value
				.get("token")
				.ok_or_else(|| Self::Error::custom("missing cookie"))?
				.value(),
		)
	}
}

/// Take a token and create a URL for it.
pub async fn token(state: &BotState, source: &Source) -> Result<Response, Response> {
	let token = Token {
		guild_id: source.guild_id,
		user_id: source.user_id,
		expiry: Utc::now() + Months::new(3),
	};

	let encrypted = Encrypted::encrypt(&token, state.data.read().await.get_expect::<AeadKey>())
		.map_err(|_| "Internal error with encrypting")?;

	let http_config = CONFIG.http.as_ref().ok_or("Http inteface not set up")?;

	let url = format!(
		"http://{}/token?{}",
		http_config.public_url,
		serde_urlencoded::to_string(encrypted)
			.map_err(|_| "Internal error with url serialization")?,
	);

	Ok(url.into())
}
