use tracing::error;

use ring::aead::{Aad, LessSafeKey, UnboundKey, AES_256_GCM};
use ring::rand::{SecureRandom, SystemRandom};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use serde_with::base64::{Base64, UrlSafe};
use serde_with::formats::Unpadded;
use serde_with::serde_as;

static ALGO: &ring::aead::Algorithm = &AES_256_GCM;

pub fn gen_key() -> LessSafeKey {
	let mut bytes = [0; 32];
	SystemRandom::new().fill(&mut bytes).unwrap();
	LessSafeKey::new(UnboundKey::new(ALGO, &bytes).unwrap())
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

impl Encrypted {
	pub fn encrypt<T: Serialize>(
		t: T,
		key: &LessSafeKey,
	) -> Result<Self, ring::error::Unspecified> {
		let mut nonce_bytes = [0; ring::aead::NONCE_LEN];
		SystemRandom::new().fill(&mut nonce_bytes).unwrap();

		let nonce = ring::aead::Nonce::assume_unique_for_key(nonce_bytes);

		let mut data = serde_json::to_vec(&t)
			.inspect_err(|e| error!("Error encrypting data: {:?}", e))
			.map_err(|_| ring::error::Unspecified)?;

		key.seal_in_place_append_tag(nonce, Aad::empty(), &mut data)?;

		Ok(Self {
			nonce: Nonce(nonce_bytes),
			data: data.into(),
		})
	}

	pub fn decrypt<T: DeserializeOwned>(
		mut self,
		key: &LessSafeKey,
	) -> Result<T, ring::error::Unspecified> {
		key.open_in_place(self.nonce.into(), Aad::empty(), &mut self.data)?;

		let object = &self.data[..self.data.len() - ALGO.tag_len()];

		serde_json::from_reader(object)
			.inspect_err(|e| error!("Error decrypting data: {:?}", e))
			.map_err(|_| ring::error::Unspecified)
	}
}
