use futures::Future;

use itertools::Itertools;

use tracing::{debug, error, warn};

use once_cell::sync::Lazy;

use regex::Regex;

use reqwest::Url;

use songbird::input::restartable::Restartable;
use songbird::input::Input;

use thiserror::Error;

use walkdir::WalkDir;

use std::borrow::Cow;
use std::path::{Path, PathBuf};

use crate::data::{ArcRw, Keys};
use crate::spotify;
use crate::util::*;
use crate::youtube::{self, YtdlLazy, YtdlSearchLazy};

static URL: Lazy<Regex> = Lazy::new(|| Regex::new("^https?://").unwrap());

static YOUTUBE_HOST: Lazy<Regex> =
	Lazy::new(|| Regex::new("^([^.]*\\.)?(youtube\\.com|youtu.be)").unwrap());

static SPOTIFY_HOST: Lazy<Regex> = Lazy::new(|| Regex::new("^open\\.spotify\\.com").unwrap());

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlayStyle {
	Play,
	Clip,
}

impl std::str::FromStr for PlayStyle {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_ascii_lowercase().as_str() {
			"play" => Ok(PlayStyle::Play),
			"clip" => Ok(PlayStyle::Clip),
			_ => Err(()),
		}
	}
}

pub fn clip_path() -> PathBuf {
	return Path::new("./resources/clips").canonicalize().unwrap();
}

#[derive(Debug, Error)]
pub enum AudioError {
	#[error("encountered songbird error: {0}")]
	Songbird(#[from] songbird::input::error::Error),
	#[error("playlists are not allowed in this context")]
	PlaylistNotAllowed,
	#[error("error using the spotify api")]
	Spotify,
	#[error("error reading youtube api for playlist")]
	YoutubePlaylist,
	#[error("unsupported url")]
	UnsupportedUrl,
	#[error("multiple clips matched")]
	MultipleClip(String, String),
	#[error("no clips matched")]
	NotFound,
}

pub async fn clip_source(loc: &str) -> Result<Input, AudioError> {
	match find_clip(&loc) {
		FindClip::One(clip) => match get_clip(&clip) {
			Some(clip) => Ok(songbird::ffmpeg(&clip).await?),
			None => Err(AudioError::NotFound),
		},
		FindClip::Multiple(clip_a, clip_b) => Err(AudioError::MultipleClip(clip_a, clip_b)),
		FindClip::None => Err(AudioError::NotFound),
	}
}

#[derive(Debug)]
pub struct SourceInfo {
	pub title: Option<String>,
	pub url: Option<String>,
	pub count: usize,
}

pub async fn play_sources<F, T>(
	keys: ArcRw<Keys>,
	loc: &str,
	allow_playlist: bool,
	f: F,
) -> Result<SourceInfo, AudioError>
where
	F: Fn(Input) -> T + Send + Sync + 'static,
	T: Future<Output = ()> + Send,
{
	if URL.is_match(loc) {
		let url = Url::parse(loc).map_err(|_| AudioError::UnsupportedUrl)?;
		let host = url.host_str().ok_or(AudioError::UnsupportedUrl)?;

		if YOUTUBE_HOST.is_match(host) {
			let path = url.path();

			let youtube_api = keys
				.read()
				.await
				.youtube
				.clone()
				.ok_or(AudioError::YoutubePlaylist)?;

			// if it is a playlist, queue the playlist
			if path == "/playlist" {
				if !allow_playlist {
					return Err(AudioError::PlaylistNotAllowed);
				}

				let id = url
					.query_pairs()
					.filter(|(key, _)| key == "list")
					.map(|(_, value)| value)
					.next()
					.ok_or_else(|| {
						debug!("Missing \"list\" in query parameters: {}", url);
						AudioError::UnsupportedUrl
					})?;

				let playlist = youtube::playlist(&youtube_api, &id)
					.await
					.map_err(|e| {
						error!("Error in youtube data api for playlists: {:?}", e);
						AudioError::YoutubePlaylist
					})?
					.ok_or_else(|| {
						error!("No playlist found");
						AudioError::NotFound
					})?;

				let playlist_items =
					youtube::playlist_items(&youtube_api, &id)
						.await
						.map_err(|e| {
							error!("Error in youtube data api for playlist items: {:?}", e);
							AudioError::YoutubePlaylist
						})?;

				let info = SourceInfo {
					title: Some(playlist.snippet.title),
					url: Some(loc.to_string()),
					count: playlist_items.len(),
				};

				tokio::spawn(async move {
					for item in playlist_items {
						match YtdlLazy::from(item).as_input().await {
							Ok(input) => f(input).await,
							Err(e) => error!("Error creating input: {:?}", e),
						}
					}
				});

				Ok(info)
			} else {
				let id = if path == "/watch" {
					url.query_pairs()
						.filter(|(key, _)| key == "v")
						.map(|(_, value)| value)
						.next()
						.ok_or_else(|| {
							debug!("Missing \"list\" in query parameters: {}", url);
							AudioError::UnsupportedUrl
						})?
				} else {
					Cow::Borrowed(&path[1..])
				};

				let video = youtube::video(&youtube_api, &id)
					.await
					.map_err(|e| {
						error!("Youtube video error: {:?}", e);
						AudioError::YoutubePlaylist
					})?
					.ok_or_else(|| {
						error!("No video found with id {:?}", id);
						AudioError::NotFound
					})?;

				let loc_string = loc.to_string();

				tokio::spawn(async move {
					match Restartable::ytdl(loc_string, true).await {
						Ok(restartable) => f(restartable.into()).await,
						Err(e) => error!("Error creating input: {:?}", e),
					}
				});

				Ok(SourceInfo {
					title: Some(video.snippet.title),
					url: Some(loc.to_string()),
					count: 1,
				})
			}
		} else if SPOTIFY_HOST.is_match(host) {
			let mut path_segments = url.path_segments().ok_or(AudioError::UnsupportedUrl)?;

			let token = keys
				.write()
				.await
				.spotify
				.as_mut()
				.ok_or(AudioError::Spotify)?
				.get_token()
				.await
				.map_err(|_| AudioError::Spotify)?
				.access_token
				.clone();

			match path_segments.next().ok_or(AudioError::UnsupportedUrl)? {
				"track" => {
					let track_id = path_segments.next().ok_or(AudioError::UnsupportedUrl)?;

					let track = spotify::track(&token, track_id).await.map_err(|e| {
						error!("Error reading spotify track: {:?}", e);
						AudioError::Spotify
					})?;

					match YtdlSearchLazy::from(&track).as_input().await {
						Ok(input) => f(input).await,
						Err(e) => error!("Error creating input: {:?}", e),
					}

					Ok(SourceInfo {
						title: Some(track.name),
						url: Some(loc.to_string()),
						count: 1,
					})
				}
				"playlist" => {
					if !allow_playlist {
						return Err(AudioError::PlaylistNotAllowed);
					}

					let playlist_id = path_segments.next().ok_or(AudioError::UnsupportedUrl)?;

					let playlist = spotify::playlist(&token, playlist_id).await.map_err(|e| {
						error!("Error reading spotify playlist: {:?}", e);
						AudioError::Spotify
					})?;

					let info = SourceInfo {
						title: Some(playlist.name.clone()),
						url: Some(loc.to_string()),
						count: playlist.tracks.items.len(),
					};

					tokio::spawn(async move {
						for track in playlist.tracks.items.into_iter().map(|t| t.track) {
							match YtdlSearchLazy::from(&track).as_input().await {
								Ok(input) => f(input).await,
								Err(e) => error!("Error creating input: {:?}", e),
							}
						}
					});

					Ok(info)
				}
				"album" => {
					if !allow_playlist {
						return Err(AudioError::PlaylistNotAllowed);
					}

					let album_id = path_segments.next().ok_or(AudioError::UnsupportedUrl)?;

					let album = spotify::album(&token, album_id).await.map_err(|e| {
						error!("Error reading spotify album: {:?}", e);
						AudioError::Spotify
					})?;

					let info = SourceInfo {
						title: Some(album.name.clone()),
						url: Some(loc.to_string()),
						count: album.tracks.items.len(),
					};

					tokio::spawn(async move {
						for track in album.tracks.items {
							match YtdlSearchLazy::from(&track).as_input().await {
								Ok(input) => f(input).await,
								Err(e) => error!("Error creating input: {:?}", e),
							}
						}
					});

					Ok(info)
				}
				_ => Err(AudioError::UnsupportedUrl),
			}
		} else {
			Err(AudioError::UnsupportedUrl)
		}
	} else {
		let loc_string = loc.to_string();

		tokio::spawn(async move {
			match Restartable::ytdl_search(loc_string, true).await {
				Ok(restartable) => f(restartable.into()).await,
				Err(e) => error!("Error creating input: {:?}", e),
			}
		});

		Ok(SourceInfo {
			title: None,
			url: None,
			count: 1,
		})
	}
}

pub enum FindClip {
	One(String),
	Multiple(String, String),
	None,
}

pub fn warn_duplicate_clip_names() {
	let clip_path = clip_path();

	WalkDir::new(&clip_path)
		.into_iter()
		.filter_map(|f| f.ok())
		.filter(|f| f.file_type().is_file())
		.map(|f| f.path().file_stem().unwrap().to_string_lossy().into_owned())
		.duplicates()
		.for_each(|s| warn!("Multiple clips have the name \"{}\"", s));
}

pub fn warn_exact_name_finds_different_clip() {
	let clip_path = clip_path();

	WalkDir::new(&clip_path)
		.into_iter()
		.filter_map(|f| f.ok())
		.filter(|f| f.file_type().is_file())
		.filter(|f| {
			let path = f.path();

			match find_clip(&path.file_stem().unwrap().to_string_lossy()) {
				FindClip::One(p) =>
					p != path
						.strip_prefix(&clip_path)
						.unwrap()
						.with_extension("")
						.to_string_lossy(),
				_ => true,
			}
		})
		.for_each(|s| warn!("Clip {:?} does not get found searching for the exact name", s.path()));
}

pub fn find_clip(loc: &str) -> FindClip {
	let clip_path = clip_path();
	//let components = loc.split('/').collect_vec();

	let top_two = WalkDir::new(&clip_path)
		.into_iter()
		.filter_map(|f| f.ok())
		.filter(|f| f.file_type().is_file())
		// calculate the levenshtein distance of each file
        // break ties by prioritizing longest length of match
        // followed by shortest length of clip path
		.filter_map(|f| {
            let path = f.path().to_string_lossy();

			let leven = triple_accel::levenshtein::levenshtein_search(
				&loc.as_bytes(),
				path.as_bytes()
			).nth(0)?;

			Some(OrdKey {
                key: (
                     leven.k,
                     -((leven.end - leven.start) as isize),
                    path.len(),
                ),
                value: f 
            })
		})
		.k_smallest(2)
		.collect_vec();

	debug!("Found the follwing top two clips: {:?}", top_two);

	if top_two.len() == 0 {
		FindClip::None
	} else if top_two.len() > 1 && top_two[0].key == top_two[1].key {
		FindClip::Multiple(
            top_two[0].value.path().strip_prefix(&clip_path).unwrap().with_extension("").to_string_lossy().into_owned(),
            top_two[1].value.path().strip_prefix(&clip_path).unwrap().with_extension("").to_string_lossy().into_owned()
        )
	} else {
		FindClip::One(
			top_two[0]
				.value
				.path()
				.strip_prefix(&clip_path)
				.unwrap()
				.with_extension("")
				.to_string_lossy()
				.into_owned(),
		)
	}
}

pub fn get_clip(loc: &str) -> Option<PathBuf> {
	let clip_path = clip_path();
	let mut play_path = clip_path.join(&loc);

	for ext in &["mp3", "wav"] {
		play_path.set_extension(ext);

		if valid_clip(&play_path) {
			return Some(play_path);
		}
	}

	None
}

pub fn valid_clip(path: &Path) -> bool {
	sandboxed_exists(&clip_path(), &path)
}
