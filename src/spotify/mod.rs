//! Structures and functions to authenticate and access the [Spotify API]
//!
//! In order to access the API, you first need a credentials struct
//! [`SpotifyApi`]. Then you can use some functions to access the API like
//! [`track`].
//!
//! [Spotify API]: https://developer.spotify.com/documentation/web-api/reference/#/

pub mod api;
pub mod scrape;

use itertools::Itertools;

use tracing::debug;

use thiserror::Error;

use serde::Deserialize;

use songbird::input::{AuxMetadata, YoutubeDl};

use std::time::{Duration, Instant};

use crate::{audio::ComposeWithMetadata, youtube::compose_yt_search_with_meta, REQWEST_CLIENT};

use api::{Album, Playlist, Response, Track};

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

#[derive(Debug, Error)]
pub enum Error {
	#[error("failed while fetching data: {0}")]
	Reqwest(#[from] reqwest::Error),
	#[error("failed to parse data: {0}")]
	Api(serde_json::Value),
	#[error("failed to scrape data: {0}")]
	Scrape(#[from] scrape::Error),
}

pub type Result<T> = core::result::Result<T, Error>;

impl SpotifyApi {
	/// If there is still a valid token, return it, and otherwise refresh the
	/// spotify token.
	pub async fn get_token(&mut self) -> Result<&SpotifyToken> {
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
	pub async fn new(api: &SpotifyApi) -> Result<Self> {
		debug!("Fetching spotify token...");

		let response = REQWEST_CLIENT
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

impl From<&Track> for ComposeWithMetadata<YoutubeDl> {
	/// Convert a spotify track into a youtube search that songbird can use.
	fn from(track: &Track) -> Self {
		let artist = if track.artists.is_empty() {
			None
		} else {
			Some(track.artists.iter().map(|a| &a.name).join(", "))
		};

		compose_yt_search_with_meta(
			format!(
				"{} {}",
				track.name,
				track.artists.iter().map(|a| &a.name).join(" ")
			),
			AuxMetadata {
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
pub async fn playlist(token: &str, playlist_id: &str) -> Result<Playlist> {
	REQWEST_CLIENT
		.get(format!(
			"https://api.spotify.com/v1/playlists/{}",
			playlist_id
		))
		.bearer_auth(token)
		.send()
		.await?
		.json::<Response<Playlist, _>>()
		.await?
		.into_result()
		.map_err(Error::Api)
}

/// Fetch and parse an album from the spotify API.
///
/// See <https://developer.spotify.com/documentation/web-api/reference/#/operations/get-an-album>
pub async fn album(token: &str, album_id: &str) -> Result<Album> {
	REQWEST_CLIENT
		.get(format!("https://api.spotify.com/v1/albums/{}", album_id))
		.bearer_auth(token)
		.send()
		.await?
		.json::<Response<Album, _>>()
		.await?
		.into_result()
		.map_err(Error::Api)
}

/// Fetch and parse a track from the spotify API.
///
/// See <https://developer.spotify.com/documentation/web-api/reference/#/operations/get-track>
pub async fn track(token: &str, track_id: &str) -> Result<Track> {
	REQWEST_CLIENT
		.get(format!("https://api.spotify.com/v1/tracks/{}", track_id))
		.bearer_auth(token)
		.send()
		.await?
		.json::<Response<Track, _>>()
		.await?
		.into_result()
		.map_err(Error::Api)
}
