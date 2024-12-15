//! Structures and functions to authenticate and access the [YouTube API]
//!
//! In order to access the API, you just need a key, defined in [`YoutubeApi`]
//!
//! [YouTube API]: https://developers.google.com/youtube/v3/docs
pub mod api;

use futures::Stream;
use itertools::Itertools;

use serde::{Deserialize, Serialize};

use songbird::constants::SAMPLE_RATE_RAW;
use songbird::input::{AuxMetadata, YoutubeDl};

use thiserror::Error;

use api::{List, Playlist, PlaylistItem, Response, Video};

use std::fmt::Display;

use crate::audio::ComposeWithMetadata;
use crate::REQWEST_CLIENT;

#[derive(Clone, Deserialize)]
pub struct YoutubeApi {
	pub key: String,
}

#[derive(Debug, Error)]
pub enum Error {
	#[error("failed while fetching data: {0}")]
	Reqwest(#[from] reqwest::Error),
	#[error("failed to parse data: {0}")]
	Api(serde_json::Value),
}

pub type Result<T> = core::result::Result<T, Error>;

pub fn compose_yt_url(
	uri: String,
	mut aux_metadata: AuxMetadata,
) -> ComposeWithMetadata<YoutubeDl> {
	aux_metadata.channels = Some(2);
	aux_metadata.sample_rate = Some(SAMPLE_RATE_RAW as u32);

	ComposeWithMetadata::new(YoutubeDl::new(REQWEST_CLIENT.clone(), uri), aux_metadata)
}

impl From<Video> for ComposeWithMetadata<YoutubeDl> {
	fn from(item: Video) -> Self {
		let url = format!("https://youtu.be/{}", item.id);
		let tn = item.snippet.thumbnails;

		compose_yt_url(
			url.clone(),
			AuxMetadata {
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

impl From<PlaylistItem> for ComposeWithMetadata<YoutubeDl> {
	fn from(item: PlaylistItem) -> Self {
		let url = format!("https://youtu.be/{}", item.content_details.video_id);
		let tn = item.snippet.thumbnails;

		compose_yt_url(
			url.clone(),
			AuxMetadata {
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

pub fn compose_yt_search(search: impl Display) -> YoutubeDl {
	fn inner(search: String) -> YoutubeDl {
		YoutubeDl::new(REQWEST_CLIENT.clone(), search)
	}

	let search = format!("ytsearch:{search}");

	inner(search)
}

pub fn compose_yt_search_with_meta(
	search: impl Display,
	metadata: AuxMetadata,
) -> ComposeWithMetadata<YoutubeDl> {
	fn inner(search: String, mut metadata: AuxMetadata) -> ComposeWithMetadata<YoutubeDl> {
		metadata.channels = Some(2);
		metadata.sample_rate = Some(SAMPLE_RATE_RAW as u32);

		ComposeWithMetadata::new(YoutubeDl::new(REQWEST_CLIENT.clone(), search), metadata)
	}

	let search = format!("ytsearch:{search}");

	inner(search, metadata)
}

pub fn stream_paged<'a, T, Q>(
	url: &'static str,
	query: Q,
) -> impl Stream<Item = Result<Vec<T>>> + 'a
where
	for<'de> T: Deserialize<'de>,
	Q: Serialize + Copy + 'a,
{
	futures::stream::unfold(Some(None), move |page_token| async move {
		if page_token.is_none() {
			None
		} else {
			let response = (|| async {
				REQWEST_CLIENT
					.get(url)
					.query(&query)
					.query(&page_token.map(|next| [("pageToken", next)]))
					.send()
					.await?
					.json::<Response<List<T>, _>>()
					.await?
					.into_result()
					.map_err(|e| Error::Api(e))
			})()
			.await;

			match response {
				Err(e) => Some((Err(e), None)),
				Ok(v) => Some((Ok(v.items), v.next_page_token.map(Some))),
			}
		}
	})
}

pub async fn playlist(api: &YoutubeApi, playlist_id: &str) -> Result<Option<Playlist>> {
	Ok(REQWEST_CLIENT
		.get("https://www.googleapis.com/youtube/v3/playlists")
		.query(&[
			("key", api.key.as_ref()),
			("part", "snippet"),
			("id", playlist_id),
		])
		.send()
		.await?
		.json::<Response<List<Playlist>, _>>()
		.await?
		.into_result()
		.map_err(|e| Error::Api(e))?
		.items
		.drain(..)
		.next())
}

pub fn playlist_items<'a>(
	api: &'a YoutubeApi,
	playlist_id: &'a str,
) -> impl Stream<Item = Result<Vec<PlaylistItem>>> + 'a {
	let url = "https://youtube.googleapis.com/youtube/v3/playlistItems";
	let query = [
		("key", api.key.as_ref()),
		("part", "contentDetails,snippet"),
		("playlistId", playlist_id),
		("maxResults", "50"),
	];

	stream_paged(url, query)
}

pub async fn video(api: &YoutubeApi, video_id: &str) -> Result<Option<Video>> {
	Ok(REQWEST_CLIENT
		.get("https://www.googleapis.com/youtube/v3/videos")
		.query(&[
			("key", api.key.as_ref()),
			("part", "contentDetails,snippet"),
			("id", video_id),
		])
		.send()
		.await?
		.json::<Response<List<Video>, _>>()
		.await?
		.into_result()
		.map_err(|e| Error::Api(e))?
		.items
		.drain(..)
		.next())
}

/// See <https://youtube.googleapis.com/youtube/v3/videos> for limitations
///
/// For example, if `video_ids` is longer than 50, this will fail
pub async fn videos(
	api: &YoutubeApi,
	video_ids: impl IntoIterator<Item = impl std::fmt::Display>,
) -> Result<Vec<Video>> {
	let url = "https://youtube.googleapis.com/youtube/v3/videos";
	let query_base = [
		("key", api.key.as_ref()),
		("part", "contentDetails,snippet"),
	];

	REQWEST_CLIENT
		.get(url)
		.query(&query_base)
		.query(&[("id", &video_ids.into_iter().join(","))])
		.send()
		.await?
		.json::<Response<List<Video>, _>>()
		.await?
		.into_result()
		.map_err(|e| Error::Api(e))
		.map(|v| v.items)
}
