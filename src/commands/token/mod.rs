use axum_extra::extract::CookieJar;

use chrono::{Months, Utc};

use ring::aead::{Aad, LessSafeKey, UnboundKey, AES_256_GCM};
use ring::rand::{SecureRandom, SystemRandom};

use serde::{de::DeserializeOwned, de::Error, Deserialize, Serialize};

use serde_with::base64::{Base64, UrlSafe};
use serde_with::formats::Unpadded;
use serde_with::serde_as;

use crate::commands::http::Token;
use crate::commands::{BotState, Source};
use crate::util::{GetExpect, Response};
use crate::AeadKey;

#[cfg(feature = "http-interface")]
pub mod http;
pub mod poise;

static ALGO: &ring::aead::Algorithm = &AES_256_GCM;

pub fn gen_key() -> LessSafeKey {
	let mut bytes = [0; 32];
	SystemRandom::new().fill(&mut bytes).unwrap();
	LessSafeKey::new(UnboundKey::new(ALGO, &bytes).unwrap())
}

pub const fn token_help() -> &'static str {
	include_str!("token.md")
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct Nonce(#[serde_as(as = "Base64<UrlSafe, Unpadded>")] [u8; ring::aead::NONCE_LEN]);

impl From<Nonce> for ring::aead::Nonce {
	fn from(n: Nonce) -> Self {
		Self::assume_unique_for_key(n.0)
	}
}

#[serde_as]
#[derive(Deserialize, Serialize)]
pub struct Encrypted {
	pub nonce: Nonce,
	#[serde_as(as = "Base64<UrlSafe, Unpadded>")]
	pub data: Box<[u8]>,
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

impl Encrypted {
	pub fn encrypt<T: Serialize>(t: T, key: &LessSafeKey) -> serde_json::Result<Self> {
		let mut nonce_bytes = [0; ring::aead::NONCE_LEN];
		SystemRandom::new().fill(&mut nonce_bytes).unwrap();

		let nonce = ring::aead::Nonce::assume_unique_for_key(nonce_bytes);

		let mut data = serde_json::to_vec(&t)?;

		key.seal_in_place_append_tag(nonce, Aad::empty(), &mut data)
			.unwrap();

		Ok(Self {
			nonce: Nonce(nonce_bytes),
			data: data.into(),
		})
	}

	pub fn decrypt<T: DeserializeOwned>(mut self, key: &LessSafeKey) -> serde_json::Result<T> {
		key.open_in_place(self.nonce.into(), Aad::empty(), &mut self.data)
			.unwrap();

		let object = &self.data[..self.data.len() - ALGO.tag_len()];

		serde_json::from_reader(object)
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

	Ok(serde_urlencoded::to_string(encrypted)
		.map_err(|_| "Internal error with url serialization")?
		.into())
}
