use reqwest::Client;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PlaylistItems {
	#[serde(rename="nextPageToken")]
	pub next_page_token: Option<String>,
	#[serde(rename="prevPageToken")]
	pub prev_page_token: Option<String>,
	#[serde(rename="pageInfo")]
	pub page_info: PageInfo,
	pub items: Vec<PlaylistItem>,
}

#[derive(Debug, Deserialize)]
pub struct PageInfo {
	#[serde(rename="totalResults")]
	pub total_results: u64,
	#[serde(rename="resultsPerPage")]
	pub results_per_page: u64,
}

#[derive(Debug, Deserialize)]
pub struct PlaylistItem {
	pub id: String,
	#[serde(rename="contentDetails")]
	pub content_details: ContentDetails,
}

#[derive(Debug, Deserialize)]
pub struct ContentDetails {
	#[serde(rename="videoId")]
	pub video_id: String,
}

pub async fn playlist(api_key: &str, playlist_id: &str) -> reqwest::Result<PlaylistItems> {
	Client::new()
		.get("https://youtube.googleapis.com/youtube/v3/playlistItems")
		.query(&[
			("part", "contentDetails"),
			("playlistId", playlist_id),
			("key", api_key),
		])
		.send()
		.await?
		.json()
		.await
}
