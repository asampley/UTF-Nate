//! Contains data structures defined by the [Spotify API].
//!
//! [Spotify API]: https://developer.spotify.com/documentation/web-api/reference/#/

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Response<T, F> {
	Parse(T),
	Fail(F),
}

impl<T, F> Response<T, F> {
	pub fn into_result(self) -> Result<T, F> {
		match self {
			Self::Parse(v) => Ok(v),
			Self::Fail(e) => Err(e),
		}
	}
}

#[derive(Debug, Deserialize)]
pub struct Playlist {
	pub name: String,
	pub tracks: PlaylistTracks,
}

#[derive(Debug, Deserialize)]
pub struct PlaylistTracks {
	pub items: Vec<PlaylistTracksItem>,
}

#[derive(Debug, Deserialize)]
pub struct PlaylistTracksItem {
	pub track: Track,
}

#[derive(Debug, Deserialize)]
pub struct Album {
	pub name: String,
	pub tracks: AlbumTracks,
}

#[derive(Debug, Deserialize)]
pub struct AlbumTracks {
	pub items: Vec<Track>,
}

#[derive(Debug, Deserialize)]
pub struct Track {
	pub id: String,
	pub name: String,
	pub duration_ms: u64,
	pub artists: Vec<Artist>,
}

#[derive(Debug, Deserialize)]
pub struct Artist {
	pub id: String,
	pub name: String,
}
