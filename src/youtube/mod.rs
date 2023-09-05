//! Structures and functions to authenticate and access the [YouTube API]
//!
//! In order to access the API, you just need a key, defined in [`YoutubeApi`]
//!
//! [YouTube API]: https://developers.google.com/youtube/v3/docs
pub mod api;

use itertools::Itertools;
use tracing::{error, info};

use reqwest::Client;

use serde::{Deserialize, Serialize};

use serenity::async_trait;

use songbird::constants::SAMPLE_RATE_RAW;
use songbird::input::codec::Codec;
use songbird::input::restartable::{Restart, Restartable};
use songbird::input::{ytdl, ytdl_search, Container, Input, Metadata};

use std::time::Duration;

use api::{ListResponse, Playlist, PlaylistItem, Video};

#[derive(Clone, Deserialize)]
pub struct YoutubeApi {
	pub key: String,
}

pub struct YtdlLazy {
	uri: String,
	metadata: Metadata,
}

pub struct YtdlSearchLazy {
	search: String,
	metadata: Metadata,
}

impl YtdlLazy {
	pub fn new(uri: String, mut metadata: Metadata) -> Self {
		metadata.channels = Some(2);
		metadata.sample_rate = Some(SAMPLE_RATE_RAW as u32);

		Self { uri, metadata }
	}

	pub async fn into_input(self) -> songbird::input::error::Result<Input> {
		self.into_restartable().await.map(|v| v.into())
	}

	pub async fn into_restartable(self) -> songbird::input::error::Result<Restartable> {
		Restartable::new(self, true).await
	}
}

impl From<Video> for YtdlLazy {
	fn from(item: Video) -> Self {
		let url = format!("https://youtu.be/{}", item.id);
		let tn = item.snippet.thumbnails;

		Self::new(
			url.clone(),
			Metadata {
				channel: Some(item.snippet.channel_title),
				source_url: Some(url),
				title: Some(item.snippet.title),
				duration: item.content_details.duration.to_std(),
				thumbnail: tn
					.default
					.or(tn.medium)
					.or(tn.high)
					.or(tn.standard)
					.or(tn.maxres)
					.map(|t| t.url),

				..Default::default()
			},
		)
	}
}

impl From<PlaylistItem> for YtdlLazy {
	fn from(item: PlaylistItem) -> Self {
		let url = format!("https://youtu.be/{}", item.content_details.video_id);
		let tn = item.snippet.thumbnails;

		Self::new(
			url.clone(),
			Metadata {
				channel: item.snippet.video_owner_channel_title,
				source_url: Some(url),
				title: Some(item.snippet.title),
				thumbnail: tn
					.default
					.or(tn.medium)
					.or(tn.high)
					.or(tn.standard)
					.or(tn.maxres)
					.map(|t| t.url),

				..Default::default()
			},
		)
	}
}

impl YtdlSearchLazy {
	pub fn new(search: String, mut metadata: Metadata) -> Self {
		metadata.channels = Some(2);
		metadata.sample_rate = Some(SAMPLE_RATE_RAW as u32);

		Self { search, metadata }
	}

	pub async fn into_input(self) -> songbird::input::error::Result<Input> {
		self.into_restartable().await.map(|v| v.into())
	}

	pub async fn into_restartable(self) -> songbird::input::error::Result<Restartable> {
		Restartable::new(self, true).await
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

#[async_trait]
impl Restart for YtdlSearchLazy {
	async fn call_restart(
		&mut self,
		_time: Option<Duration>,
	) -> songbird::input::error::Result<Input> {
		let input = ytdl_search(&self.search).await?;

		match input.metadata.source_url {
			Some(ref url) => info!("Youtube lazy search \"{}\" found {}", self.search, url),
			None => error!("Youtube lazy search \"{}\" URL not set", self.search),
		}

		Ok(input)
	}

	async fn lazy_init(
		&mut self,
	) -> songbird::input::error::Result<(Option<Metadata>, Codec, Container)> {
		Ok((Some(self.metadata.clone()), Codec::FloatPcm, Container::Raw))
	}
}

pub async fn collect_paged<T, Q>(url: &str, query: &Q) -> reqwest::Result<Vec<T>>
where
	for<'a> T: Deserialize<'a>,
	Q: Serialize + ?Sized,
{
	let mut collect = Vec::new();

	let client = Client::new();

	let mut response = client
		.get(url)
		.query(query)
		.send()
		.await?
		.json::<ListResponse<T>>()
		.await?;

	collect.append(&mut response.items);

	while let Some(next) = response.next_page_token {
		response = client
			.get(url)
			.query(query)
			.query(&[("pageToken", next)])
			.send()
			.await?
			.json::<ListResponse<T>>()
			.await?;

		collect.append(&mut response.items);
	}

	Ok(collect)
}

pub async fn collect_batch<T, Q, A>(
	url: &str,
	query_base: &Q,
	queries: impl Iterator<Item = A>,
) -> reqwest::Result<Vec<T>>
where
	for<'a> T: Deserialize<'a>,
	Q: Serialize + ?Sized,
	A: Serialize,
{
	let mut collect = Vec::new();

	let client = Client::new();

	for query in queries {
		let mut response = client
			.get(url)
			.query(query_base)
			.query(&query)
			.send()
			.await?
			.json::<ListResponse<T>>()
			.await?;

		collect.append(&mut response.items);
	}

	Ok(collect)
}

pub async fn playlist(api: &YoutubeApi, playlist_id: &str) -> reqwest::Result<Option<Playlist>> {
	Ok(Client::new()
		.get("https://www.googleapis.com/youtube/v3/playlists")
		.query(&[
			("key", api.key.as_ref()),
			("part", "snippet"),
			("id", playlist_id),
		])
		.send()
		.await?
		.json::<ListResponse<Playlist>>()
		.await?
		.items
		.drain(..)
		.next())
}

pub async fn playlist_items(
	api: &YoutubeApi,
	playlist_id: &str,
) -> reqwest::Result<Vec<PlaylistItem>> {
	let url = "https://youtube.googleapis.com/youtube/v3/playlistItems";
	let query = &[
		("key", api.key.as_ref()),
		("part", "contentDetails,snippet"),
		("playlistId", playlist_id),
		("maxResults", "50"),
	];

	collect_paged(url, query).await
}

pub async fn video(api: &YoutubeApi, video_id: &str) -> reqwest::Result<Option<Video>> {
	Ok(Client::new()
		.get("https://www.googleapis.com/youtube/v3/videos")
		.query(&[
			("key", api.key.as_ref()),
			("part", "contentDetails,snippet"),
			("id", video_id),
		])
		.send()
		.await?
		.json::<ListResponse<Video>>()
		.await?
		.items
		.drain(..)
		.next())
}

pub async fn videos(
	api: &YoutubeApi,
	video_ids: impl IntoIterator<Item = impl std::fmt::Display>,
) -> reqwest::Result<Vec<Video>> {
	let url = "https://youtube.googleapis.com/youtube/v3/videos";
	let query_base = &[
		("key", api.key.as_ref()),
		("part", "contentDetails,snippet"),
	];

	let mut video_ids = video_ids.into_iter();

	collect_batch(
		url,
		query_base,
		std::iter::from_fn(|| {
			let ids = video_ids.by_ref().take(50).join(",");
			if ids.is_empty() {
				None
			} else {
				Some([("id", ids)])
			}
		}),
	)
	.await
}
