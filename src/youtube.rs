use reqwest::Client;

use serde::Deserialize;

use serenity::async_trait;

use songbird::constants::SAMPLE_RATE_RAW;
use songbird::input::codec::Codec;
use songbird::input::restartable::{Restart, Restartable};
use songbird::input::{ytdl, Container, Input, Metadata};

use std::time::Duration;

pub struct YtdlLazy {
	uri: String,
	metadata: Metadata,
}

impl YtdlLazy {
	pub async fn as_input(self) -> songbird::input::error::Result<Input> {
		self.as_restartable().await.map(|v| v.into())
	}

	pub async fn as_restartable(self) -> songbird::input::error::Result<Restartable> {
		Restartable::new(self, true).await
	}

	pub fn from_item(item: &PlaylistItem) -> Self {
		Self {
			uri: format!("https://youtu.be/{}", item.content_details.video_id),
			metadata: Metadata {
				track: None,
				artist: None,
				date: None,
				channels: Some(2),
				channel: None,
				duration: None,
				sample_rate: Some(SAMPLE_RATE_RAW as u32),
				source_url: None,
				title: None,
				thumbnail: None,

				..Default::default()
			},
		}
	}
}

#[async_trait]
impl Restart for YtdlLazy {
	async fn call_restart(
		&mut self,
		_time: Option<Duration>,
	) -> songbird::input::error::Result<Input> {
		ytdl(&self.uri).await
	}

	async fn lazy_init(
		&mut self,
	) -> songbird::input::error::Result<(Option<Metadata>, Codec, Container)> {
		Ok((Some(self.metadata.clone()), Codec::FloatPcm, Container::Raw))
	}
}

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

pub async fn playlist(api_key: &str, playlist_id: &str) -> reqwest::Result<PlaylistItems> {
	Client::new()
		.get("https://youtube.googleapis.com/youtube/v3/playlistItems")
		.query(&[
			("part", "contentDetails"),
			("playlistId", playlist_id),
			("maxResults", "50"),
			("key", api_key),
		])
		.send()
		.await?
		.json()
		.await
}
