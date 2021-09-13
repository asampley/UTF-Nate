pub mod api;

use itertools::Itertools;

use reqwest::Client;

use serenity::async_trait;

use songbird::constants::SAMPLE_RATE_RAW;
use songbird::input::codec::Codec;
use songbird::input::restartable::{Restart, Restartable};
use songbird::input::{ytdl, ytdl_search, Container, Input, Metadata};

use std::time::Duration;

use crate::spotify::api::Track;

use api::{PlaylistItems, Video, VideoList};

pub struct YtdlLazy {
	uri: String,
	metadata: Metadata,
}

pub struct YtdlSearchLazy {
	search: String,
	metadata: Metadata,
}

impl YtdlLazy {
	pub async fn as_input(self) -> songbird::input::error::Result<Input> {
		self.as_restartable().await.map(|v| v.into())
	}

	pub async fn as_restartable(self) -> songbird::input::error::Result<Restartable> {
		Restartable::new(self, true).await
	}

	pub fn from_video(video: Video) -> Self {
		let url = format!("https://youtu.be/{}", video.id);
		let tn = video.snippet.thumbnails;

		Self {
			uri: url.clone(),
			metadata: Metadata {
				track: None,
				artist: None,
				date: None,
				channels: Some(2),
				channel: Some(video.snippet.channel_title),
				duration: None,
				sample_rate: Some(SAMPLE_RATE_RAW as u32),
				source_url: Some(url),
				title: Some(video.snippet.title),
				thumbnail: tn
					.default
					.or(tn.medium)
					.or(tn.high)
					.or(tn.standard)
					.or(tn.maxres)
					.map(|t| t.url),

				..Default::default()
			},
		}
	}
}

impl YtdlSearchLazy {
	pub async fn as_input(self) -> songbird::input::error::Result<Input> {
		self.as_restartable().await.map(|v| v.into())
	}

	pub async fn as_restartable(self) -> songbird::input::error::Result<Restartable> {
		Restartable::new(self, true).await
	}

	pub fn from_track(track: &Track) -> Self {
		let artist = if track.artists.len() == 0 {
			None
		} else {
			Some(track.artists.iter().map(|a| &a.name).join(", "))
		};

		Self {
			search: format!(
				"{} {}",
				track.name,
				track.artists.iter().map(|a| &a.name).join(" ")
			),
			metadata: Metadata {
				title: Some(track.name.clone()),
				artist: artist,
				channels: Some(2),
				sample_rate: Some(SAMPLE_RATE_RAW as u32),

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

#[async_trait]
impl Restart for YtdlSearchLazy {
	async fn call_restart(
		&mut self,
		_time: Option<Duration>,
	) -> songbird::input::error::Result<Input> {
		ytdl_search(&self.search).await
	}

	async fn lazy_init(
		&mut self,
	) -> songbird::input::error::Result<(Option<Metadata>, Codec, Container)> {
		Ok((Some(self.metadata.clone()), Codec::FloatPcm, Container::Raw))
	}
}

pub async fn videos(api_key: &str, playlist_id: &str) -> reqwest::Result<VideoList> {
	let playlist = Client::new()
		.get("https://youtube.googleapis.com/youtube/v3/playlistItems")
		.query(&[
			("key", api_key),
			("part", "contentDetails"),
			("playlistId", playlist_id),
			("maxResults", "50"),
		])
		.send()
		.await?
		.json::<PlaylistItems>()
		.await?;

	Client::new()
		.get("https://www.googleapis.com/youtube/v3/videos")
		.query(&[
			("key", api_key),
			("part", "contentDetails,snippet,id"),
			(
				"id",
				&playlist
					.items
					.into_iter()
					.map(|i| i.content_details.video_id)
					.join(","),
			),
			("maxResults", "50"),
		])
		.send()
		.await?
		.json()
		.await
}
