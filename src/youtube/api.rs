use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PlaylistItems {
	#[serde(rename = "nextPageToken")]
	pub next_page_token: Option<String>,
	#[serde(rename = "prevPageToken")]
	pub prev_page_token: Option<String>,
	#[serde(rename = "pageInfo")]
	pub page_info: PageInfo,
	pub items: Vec<PlaylistItem>,
}

#[derive(Debug, Deserialize)]
pub struct PageInfo {
	#[serde(rename = "totalResults")]
	pub total_results: u64,
	#[serde(rename = "resultsPerPage")]
	pub results_per_page: u64,
}

#[derive(Debug, Deserialize)]
pub struct PlaylistItem {
	pub id: String,
	#[serde(rename = "contentDetails")]
	pub content_details: ContentDetails,
}

#[derive(Debug, Deserialize)]
pub struct ContentDetails {
	#[serde(rename = "videoId")]
	pub video_id: String,
}

#[derive(Debug, Deserialize)]
pub struct VideoList {
	#[serde(rename = "nextPageToken")]
	pub next_page_token: Option<String>,
	#[serde(rename = "prevPageToken")]
	pub prev_page_token: Option<String>,
	#[serde(rename = "pageInfo")]
	pub page_info: PageInfo,
	pub items: Vec<Video>,
}

#[derive(Debug, Deserialize)]
pub struct Video {
	pub id: String,
	pub snippet: VideoSnippet,
	#[serde(rename = "contentDetails")]
	pub content_details: VideoContentDetails,
}

#[derive(Debug, Deserialize)]
pub struct VideoSnippet {
	pub title: String,
	#[serde(rename = "channelTitle")]
	pub channel_title: String,
	pub thumbnails: Thumbnails,
}

#[derive(Debug, Deserialize)]
pub struct VideoContentDetails {
	pub duration: String,
}

#[derive(Debug, Deserialize)]
pub struct Thumbnails {
	pub default: Option<Thumbnail>,
	pub medium: Option<Thumbnail>,
	pub high: Option<Thumbnail>,
	pub standard: Option<Thumbnail>,
	pub maxres: Option<Thumbnail>,
}

#[derive(Debug, Deserialize)]
pub struct Thumbnail {
	pub url: String,
	pub width: u64,
	pub height: u64,
}
