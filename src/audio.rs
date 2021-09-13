use futures::Future;

use itertools::Itertools;

use log::{debug, error, warn};

use once_cell::sync::Lazy;

use regex::Regex;

use reqwest::Url;

use songbird::input::restartable::Restartable;
use songbird::input::Input;

use walkdir::WalkDir;

use std::cmp;
use std::fmt;
use std::path::{Path, PathBuf};

use crate::data::{ArcRw, Keys};
use crate::spotify;
use crate::util::*;
use crate::youtube;

static URL: Lazy<Regex> = Lazy::new(|| Regex::new("^https?://").unwrap());

static YOUTUBE_HOST: Lazy<Regex> =
	Lazy::new(|| Regex::new("^((www|m)\\.youtube\\.com|youtu.be)").unwrap());

static SPOTIFY_HOST: Lazy<Regex> = Lazy::new(|| Regex::new("^open\\.spotify\\.com").unwrap());

#[derive(Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug)]
pub enum AudioError {
	Songbird(songbird::input::error::Error),
	Spotify,
	YoutubePlaylist,
	UnsupportedUrl,
	MultipleClip,
	NoClip,
}

impl From<songbird::input::error::Error> for AudioError {
	fn from(e: songbird::input::error::Error) -> Self {
		AudioError::Songbird(e)
	}
}

impl fmt::Display for AudioError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fmt::Debug::fmt(self, f)
	}
}

impl std::error::Error for AudioError {}

pub async fn clip_source(loc: &str) -> Result<Input, AudioError> {
	match find_clip(&loc) {
		FindClip::One(clip) => match get_clip(&clip) {
			Some(clip) => Ok(songbird::ffmpeg(&clip).await?),
			None => Err(AudioError::NoClip),
		},
		FindClip::Multiple => Err(AudioError::MultipleClip),
		FindClip::None => Err(AudioError::NoClip),
	}
}

pub async fn play_sources<F, T>(keys: ArcRw<Keys>, loc: &str, f: F) -> Result<usize, AudioError>
where
	F: Fn(Input) -> T + Send + Sync + 'static,
	T: Future + Send,
{
	if URL.is_match(loc) {
		let url = Url::parse(loc).map_err(|_| AudioError::UnsupportedUrl)?;
		let host = url.host_str().ok_or(AudioError::UnsupportedUrl)?;

		if YOUTUBE_HOST.is_match(host) {
			let path = url.path();

			// if it is a playlist, queue the playlist
			if path == "/playlist" {
				let id = url
					.query_pairs()
					.filter(|(key, _)| key == "list")
					.map(|(_, value)| value)
					.next()
					.ok_or(AudioError::UnsupportedUrl)?;

				let youtube_api = keys
					.read()
					.await
					.youtube_api
					.clone()
					.ok_or(AudioError::YoutubePlaylist)?;

				let videos = youtube::videos(&youtube_api, &id).await;

				debug!("Youtube videos: {:#?}", videos);

				let videos = videos.map_err(|_| AudioError::YoutubePlaylist)?;

				let count = videos.items.len();

				tokio::spawn(async move {
					for video in videos.items {
						let result = youtube::YtdlLazy::from_video(video).as_input().await;

						match result {
							Ok(input) => {
								f(input).await;
							}
							Err(e) => error!("Error creating input: {:?}", e),
						}
					}
				});

				Ok(count)
			} else {
				let loc_string = loc.to_string();
				tokio::spawn(async move {
					let result = Restartable::ytdl(loc_string, true).await;

					match result {
						Ok(restartable) => {
							f(restartable.into()).await;
						}
						Err(e) => error!("Error creating input: {:?}", e),
					}
				});
				Ok(1)
			}
		} else if SPOTIFY_HOST.is_match(host) {
			let mut path_segments = url.path_segments().ok_or(AudioError::UnsupportedUrl)?;

			let token = keys
				.write()
				.await
				.spotify_api
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

					let track = spotify::track(&token, track_id).await;

					debug!("Spotify track: {:?}", track);

					let track = track.map_err(|_| AudioError::Spotify)?;

					let result = youtube::YtdlSearchLazy::from_track(&track).as_input().await;

					match result {
						Ok(input) => {
							f(input).await;
						}
						Err(e) => error!("Error creating input: {:?}", e),
					}

					Ok(1)
				}
				"playlist" => {
					let playlist_id = path_segments.next().ok_or(AudioError::UnsupportedUrl)?;

					let playlist = spotify::playlist(&token, playlist_id).await;

					debug!("Spotify playlist: {:?}", playlist);

					let playlist = playlist.map_err(|_| AudioError::Spotify)?;

					let count = playlist.tracks.items.len();

					tokio::spawn(async move {
						for track in playlist.tracks.items.into_iter().map(|t| t.track) {
							let result =
								youtube::YtdlSearchLazy::from_track(&track).as_input().await;

							match result {
								Ok(input) => {
									f(input).await;
								}
								Err(e) => error!("Error creating input: {:?}", e),
							}
						}
					});

					Ok(count)
				}
				"album" => {
					let album_id = path_segments.next().ok_or(AudioError::UnsupportedUrl)?;

					let album = spotify::album(&token, album_id).await;

					debug!("Spotify album {:?}", album);

					let album = album.map_err(|_| AudioError::Spotify)?;

					let count = album.tracks.items.len();

					tokio::spawn(async move {
						for track in album.tracks.items {
							let result =
								youtube::YtdlSearchLazy::from_track(&track).as_input().await;

							match result {
								Ok(input) => {
									f(input).await;
								}
								Err(e) => error!("Error creating input: {:?}", e),
							}
						}
					});

					Ok(count)
				}
				_ => Err(AudioError::UnsupportedUrl),
			}
		} else {
			Err(AudioError::UnsupportedUrl)
		}
	} else {
		let loc_string = loc.to_string();
		tokio::spawn(async move {
			let result = Restartable::ytdl_search(loc_string, true).await;

			match result {
				Ok(restartable) => {
					f(restartable.into()).await;
				}
				Err(e) => error!("Error creating input: {:?}", e),
			}
		});
		Ok(1)
	}
}

pub enum FindClip {
	One(String),
	Multiple,
	None,
}

#[derive(Debug)]
struct OrdKey<K, V> {
	key: K,
	value: V,
}

impl<K, V> PartialEq for OrdKey<K, V>
where
	K: PartialEq,
{
	fn eq(&self, other: &Self) -> bool {
		self.key.eq(&other.key)
	}
}

impl<K, V> Eq for OrdKey<K, V> where K: Eq {}

impl<K, V> PartialOrd for OrdKey<K, V>
where
	K: PartialOrd,
{
	fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
		self.key.partial_cmp(&other.key)
	}
}

impl<K, V> Ord for OrdKey<K, V>
where
	K: Ord,
{
	fn cmp(&self, other: &Self) -> cmp::Ordering {
		self.key.cmp(&other.key)
	}
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

pub fn find_clip(loc: &str) -> FindClip {
	let clip_path = clip_path();
	let components = loc.split('/').collect_vec();

	let top_two = WalkDir::new(&clip_path)
		.into_iter()
		.filter_map(|f| f.ok())
		.filter(|f| f.file_type().is_file())
		// The name of the clip must match exactly in the path
		.filter(|f| {
			components
				.iter()
				.contains(&&*f.path().file_stem().unwrap().to_string_lossy())
		})
		// count the number of components in a path which match the supplied components
		// the highest score becomes the clip
		// a tie results in no clip returned
		.map(|f| OrdKey {
			key: -(f
				.path()
				.components()
				.filter(|c| {
					components
						.iter()
						.any(|d| d == &c.as_os_str().to_string_lossy())
				})
				.count() as isize),
			value: f,
		})
		.k_smallest(2)
		.collect_vec();

	debug!("Found the follwing top two clips: {:?}", top_two);

	if top_two.len() == 0 {
		FindClip::None
	} else if top_two.len() > 1 && top_two[0].key == top_two[1].key {
		FindClip::Multiple
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
