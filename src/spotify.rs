pub mod api;

use log::debug;

use reqwest::Client;

use serde::Deserialize;

use std::time::{Duration, Instant};

use api::{Album, Playlist, Track};

#[derive(Deserialize)]
pub struct SpotifyApi {
	pub client_id: String,
	pub client_secret: String,
	#[serde(skip)]
	pub token: Option<SpotifyToken>,
}

impl SpotifyApi {
	pub async fn get_token(&mut self) -> reqwest::Result<&SpotifyToken> {
		if self.token.as_ref().map(|t| t.expired()).unwrap_or(true) {
			debug!("Refreshing spotify token");
			self.token = Some(SpotifyToken::new(self).await?);
			debug!("New token: {:?}", self.token);
		}
		Ok(self.token.as_ref().unwrap())
	}
}

#[derive(Debug, Deserialize)]
pub struct SpotifyToken {
	pub access_token: String,
	token_type: String,
	expires_in: u64,
	#[serde(skip)]
	refresh_after: Option<Instant>,
}

impl SpotifyToken {
	pub async fn new(api: &SpotifyApi) -> reqwest::Result<Self> {
		debug!("Fetching spotify token...");

		let response = reqwest::Client::new()
			.post("https://accounts.spotify.com/api/token")
			.basic_auth(&api.client_id, Some(&api.client_secret))
			.header("content-type", "application/x-www-form-urlencoded")
			.body("grant_type=client_credentials")
			.send()
			.await?;

		debug!("Spotify reponse {:?}", response);

		let mut token = response.json::<Self>().await?;

		token.set_refresh_time(Duration::from_secs(10));

		Ok(token)
	}

	fn set_refresh_time(&mut self, buffer_time: Duration) {
		self.refresh_after =
			Some(Instant::now() + Duration::from_secs(self.expires_in) - buffer_time);
	}

	fn expired(&self) -> bool {
		self.refresh_after
			.map(|t| t < Instant::now())
			.unwrap_or(true)
	}
}

pub async fn playlist(token: &str, playlist_id: &str) -> reqwest::Result<Playlist> {
	Client::new()
		.get(format!(
			"https://api.spotify.com/v1/playlists/{}",
			playlist_id
		))
		.bearer_auth(token)
		.send()
		.await?
		.json()
		.await
}

pub async fn album(token: &str, album_id: &str) -> reqwest::Result<Album> {
	Client::new()
		.get(format!("https://api.spotify.com/v1/albums/{}", album_id))
		.bearer_auth(token)
		.send()
		.await?
		.json()
		.await
}

pub async fn track(token: &str, track_id: &str) -> reqwest::Result<Track> {
	Client::new()
		.get(format!("https://api.spotify.com/v1/tracks/{}", track_id))
		.bearer_auth(token)
		.send()
		.await?
		.json()
		.await
}
