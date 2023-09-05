use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListResponse<T> {
	pub next_page_token: Option<String>,
	pub prev_page_token: Option<String>,
	pub page_info: PageInfo,
	pub items: Vec<T>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
	pub total_results: u64,
	pub results_per_page: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Playlist {
	pub id: String,
	pub snippet: PlaylistSnippet,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistSnippet {
	pub title: String,
	pub channel_title: String,
	pub thumbnails: Thumbnails,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistItem {
	pub id: String,
	pub snippet: PlaylistItemSnippet,
	pub content_details: ContentDetails,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistItemSnippet {
	pub title: String,
	pub channel_title: String,
	pub video_owner_channel_title: Option<String>,
	pub video_owner_channel_id: Option<String>,
	pub thumbnails: Thumbnails,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentDetails {
	pub video_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoList {
	pub next_page_token: Option<String>,
	pub prev_page_token: Option<String>,
	pub page_info: PageInfo,
	pub items: Vec<Video>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Video {
	pub id: String,
	pub snippet: VideoSnippet,
	pub content_details: VideoContentDetails,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoSnippet {
	pub title: String,
	pub channel_title: String,
	pub thumbnails: Thumbnails,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoContentDetails {
	pub duration: iso8601_duration::Duration,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnails {
	pub default: Option<Thumbnail>,
	pub medium: Option<Thumbnail>,
	pub high: Option<Thumbnail>,
	pub standard: Option<Thumbnail>,
	pub maxres: Option<Thumbnail>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnail {
	pub url: String,
	pub width: u64,
	pub height: u64,
}
