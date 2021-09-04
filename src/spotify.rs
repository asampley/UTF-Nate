use librespot::audio::AudioPacket;
use librespot::core::authentication::Credentials;
use librespot::core::config::SessionConfig;
use librespot::core::session::Session;
use librespot::core::session::SessionError;
use librespot::playback::audio_backend::Sink;
use librespot::playback::config::PlayerConfig;
use librespot::playback::player::{Player, PlayerEventChannel};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserPassword {
	pub username: String,
	pub password: String,
}

struct PrintSink;

impl Sink for PrintSink {
	fn start(&mut self) -> std::io::Result<()> {
		Ok(())
	}

	fn stop(&mut self) -> std::io::Result<()> {
		Ok(())
	}

	fn write(&mut self, packet: &AudioPacket) -> std::io::Result<()> {
		match packet {
			AudioPacket::Samples(s) => println!("Samples({:?})", s),
			AudioPacket::OggData(s) => println!("OggData({:?})", s),
		}

		Ok(())
	}
}

pub async fn session(credentials: UserPassword) -> Result<Session, SessionError> {
	Session::connect(
		SessionConfig::default(),
		Credentials::with_password(credentials.username, credentials.password),
		None,
	).await
}

pub async fn player(session: Session) -> (Player, PlayerEventChannel) {
	Player::new(
		PlayerConfig::default(),
		session,
		None,
		|| Box::new(PrintSink),
	)
}
