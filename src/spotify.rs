//! Structures and functions to authenticate and access the [Spotify API]
//!
//! In order to access the API, you first need a credentials struct
//! [`SpotifyApi`]. Then you can use some functions to access the API like
//! [`track`].
//!
//! [Spotify API]: https://developer.spotify.com/documentation/web-api/reference/#/

pub mod api;

use itertools::Itertools;

use tracing::debug;

use reqwest::Client;

use serde::Deserialize;

use songbird::input::Metadata;

use std::time::{Duration, Instant};

use crate::youtube::YtdlSearchLazy;

use api::{Album, Playlist, Track};

/// Information required to connect to the Spotify API.
///
/// [Spotify authorization walkthrough](
///     https://developer.spotify.com/documentation/general/guides/authorization/
/// )
///
/// Tokens are generated periodicallly, and must be refreshed, so are not
/// serialized.
#[derive(Deserialize)]
pub struct SpotifyApi {
	pub client_id: String,
	pub client_secret: String,
	#[serde(skip)]
	pub token: Option<SpotifyToken>,
}

impl SpotifyApi {
	/// If there is still a valid token, return it, and otherwise refresh the
	/// spotify token.
	pub async fn get_token(&mut self) -> reqwest::Result<&SpotifyToken> {
		if self.token.as_ref().map(|t| t.expired()).unwrap_or(true) {
			debug!("Refreshing spotify token");
			self.token = Some(SpotifyToken::new(self).await?);
			debug!("New token: {:?}", self.token);
		}
		Ok(self.token.as_ref().unwrap())
	}
}

/// Expiring token for the spotify API. Required to connect, but must be
/// refreshed periodically if the `refresh_after` time has passed.
#[derive(Debug, Deserialize)]
pub struct SpotifyToken {
	pub access_token: String,
	#[allow(dead_code)]
	token_type: String,
	expires_in: u64,
	#[serde(skip)]
	refresh_after: Option<Instant>,
}

impl SpotifyToken {
	/// Create a new token using persistent credentials.
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

	/// Set the refresh time of the token with slightly less time than
	/// than `expires_in` indicates, to refresh before it expires.
	///
	/// How long before the expiry time this token should be refreshed is set
	/// by `buffer_time`.
	fn set_refresh_time(&mut self, buffer_time: Duration) {
		self.refresh_after =
			Some(Instant::now() + Duration::from_secs(self.expires_in) - buffer_time);
	}

	/// Returns true if the current time has passed the `refresh_after` time.
	fn expired(&self) -> bool {
		self.refresh_after
			.map(|t| t < Instant::now())
			.unwrap_or(true)
	}
}

impl From<&Track> for YtdlSearchLazy {
	/// Convert a spotify track into a youtube search that songbird can use.
	fn from(track: &Track) -> Self {
		let artist = if track.artists.is_empty() {
			None
		} else {
			Some(track.artists.iter().map(|a| &a.name).join(", "))
		};

		Self::new(
			format!(
				"{} {}",
				track.name,
				track.artists.iter().map(|a| &a.name).join(" ")
			),
			Metadata {
				title: Some(track.name.clone()),
				artist,
				source_url: Some(format!("https://open.spotify.com/track/{}", track.id)),

				..Default::default()
			},
		)
	}
}

/// Fetch and parse a playlist from the spotify API.
///
/// See <https://developer.spotify.com/documentation/web-api/reference/#/operations/get-playlist>
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

/// Fetch and parse an album from the spotify API.
///
/// See <https://developer.spotify.com/documentation/web-api/reference/#/operations/get-an-album>
pub async fn album(token: &str, album_id: &str) -> reqwest::Result<Album> {
	Client::new()
		.get(format!("https://api.spotify.com/v1/albums/{}", album_id))
		.bearer_auth(token)
		.send()
		.await?
		.json()
		.await
}

/// Fetch and parse a track from the spotify API.
///
/// See <https://developer.spotify.com/documentation/web-api/reference/#/operations/get-track>
pub async fn track(token: &str, track_id: &str) -> reqwest::Result<Track> {
	Client::new()
		.get(format!("https://api.spotify.com/v1/tracks/{}", track_id))
		.bearer_auth(token)
		.send()
		.await?
		.json()
		.await
}
