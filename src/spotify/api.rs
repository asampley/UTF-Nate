use serde::Deserialize;

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
	pub preview_url: Option<String>,
	pub artists: Vec<Artist>,
}

#[derive(Debug, Deserialize)]
pub struct Artist {
	pub id: String,
	pub name: String,
}
