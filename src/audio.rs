//! Fetch Spotify, YouTube, online audio files, and locally stored clips for
//! playing.
//!
//! Ultimately all audio sources that are streamed are from youtube, but the
//! metadata from Spotify is parsed in order to create a search on youtube
//! which will likely return the source being searched for.
//!
//! Clip searches are done using levenshtein distance in order to fuzzily
//! match making it easier to use without knowing exact clip names.

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
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

use crate::data::{ArcRw, Keys};
use crate::spotify;
use crate::util::*;
use crate::youtube::{self, YtdlLazy, YtdlSearchLazy};
use crate::RESOURCE_PATH;

/// Path to shared directory for clips.
pub static CLIP_PATH: Lazy<PathBuf> = Lazy::new(|| RESOURCE_PATH.join("clips/"));

/// Regular expression which matches valid http or https urls.
static URL: Lazy<Regex> = Lazy::new(|| Regex::new("^https?://").unwrap());

/// Regular expression which matches the host portion of a url if the host is youtube.
static YOUTUBE_HOST: Lazy<Regex> =
	Lazy::new(|| Regex::new("^([^.]*\\.)?(youtube\\.com|youtu.be)").unwrap());

/// Regular expression which matches the host portion of a url if the host is spotify.
static SPOTIFY_HOST: Lazy<Regex> = Lazy::new(|| Regex::new("^open\\.spotify\\.com").unwrap());

/// Enum for the two styles of audio source.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlayStyle {
	/// Remote streamed audio sources that are queued.
	Play,
	/// Local audio clips that are played immediately.
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

/// Error type for all audio errors in fetching clips or playing clips.
#[derive(Debug, Error)]
pub enum AudioError {
	/// Pass through to a songbird error.
	#[error("encountered songbird error: {0}")]
	Songbird(#[from] songbird::input::error::Error),

	/// Error indicating the context does not allow playlists.
	#[error("playlists are not allowed in this context")]
	PlaylistNotAllowed,

	/// Generic error when using spotify.
	#[error("error using the spotify api")]
	Spotify,

	/// Generic error when fetching a youtube playlist.
	#[error("error reading youtube api for playlist")]
	YoutubePlaylist,

	/// Error indicating that the url that was supplied cannot be used to get clips.
	#[error("unsupported url")]
	UnsupportedUrl,

	/// Error indicating too many matching clips were found.
	#[error("multiple clips matched")]
	MultipleClip(OsString, OsString),

	/// Error indicating no matching clips were found.
	#[error("no clips matched")]
	NotFound,
}

/// Create an audio source for a clip based on a search string.
///
/// The search functionality is based on the [`find_clip`] function, and this
/// merely converts it into a playable source for [`songbird`].
pub async fn clip_source(loc: &OsStr) -> Result<Input, AudioError> {
	match find_clip(loc) {
		FindClip::One(clip) => match get_clip(&clip) {
			Some(clip) => Ok(songbird::ffmpeg(&clip).await?),
			None => Err(AudioError::NotFound),
		},
		FindClip::Multiple(clip_a, clip_b) => Err(AudioError::MultipleClip(clip_a, clip_b)),
		FindClip::None => Err(AudioError::NotFound),
	}
}

/// Contains a few details for a source for display. This could be a single source
/// or a playlist, but either way should have a title and url, if possible.
#[derive(Debug)]
pub struct SourceInfo {
	/// The title that can be used as a good display string.
	pub title: Option<String>,

	/// The url which actually directs to the resource.
	pub url: Option<String>,

	/// The number of actual sources, if a playlist.
	pub count: usize,
}

/// Creates audio sources for [`songbird`], and as each audio source is created
/// it is run through the supplied callback `f`. Information about the audio
/// sources is returned upon completion of all callbacks.
///
/// If `loc` is a URL, as matched by the [`URL`] regular expression, it will try
/// to match either a youtube or spotify URL, based on the host names in the
/// [`YOUTUBE_HOST`] and [`SPOTIFY_HOST`] regular expressions. Any other URL
/// will be run through ffmpeg, and if that fails, it returns an
/// [`AudioError::UnsupportedUrl`] error.
///
/// If `loc` is not a URL then it will instead do a search on youtube and grab
/// the first match.
///
/// Certain contexts may wish to exclude playlists, so `allow_playlist` can be
/// used to return an [`AudioError::PlaylistNotAllowed`] instead.
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
						match YtdlLazy::from(item).into_input().await {
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

					match YtdlSearchLazy::from(&track).into_input().await {
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
							match YtdlSearchLazy::from(&track).into_input().await {
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
							match YtdlSearchLazy::from(&track).into_input().await {
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
			match songbird::ffmpeg(loc).await {
				Ok(source) => {
					f(source).await;

					Ok(SourceInfo {
						title: None,
						url: Some(loc.to_string()),
						count: 1,
					})
				}
				Err(e) => {
					error!("Error creating input: {:?}", e);
					Err(AudioError::UnsupportedUrl)
				}
			}
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

/// Results from searching for a clip.
pub enum FindClip {
	/// A single best matching clip was found.
	One(OsString),

	/// At least two equally matching clips were found.
	Multiple(OsString, OsString),

	/// No matching clips were found.
	None,
}

/// Log a warning if any clips have the same file stem.
///
/// Different extensions are ignored, the comparison is done using
/// [`Path::file_stem`]. The messages are logged using [`warn!()`].
pub fn warn_duplicate_clip_names() {
	WalkDir::new(&*CLIP_PATH)
		.into_iter()
		.filter_map(|f| f.ok())
		.filter(|f| f.file_type().is_file())
		.filter_map(|f| {
			f.path()
				.file_stem()
				.unwrap()
				.to_str()
				.map(ToOwned::to_owned)
		})
		.duplicates()
		.for_each(|s| warn!("Multiple clips have the name \"{}\"", s));
}

/// Log a warning if any clips are not found with the exact clip name, ignoring
/// directories.
///
/// Different extensions are ignored, the comparison is done using
/// [`Path::file_stem`]. The messages are logged using [`warn!()`].
pub fn warn_exact_name_finds_different_clip() {
	WalkDir::new(&*CLIP_PATH)
		.into_iter()
		.filter_map(|f| f.ok())
		.filter(|f| f.file_type().is_file())
		.filter(|f| {
			let path = f.path();

			match find_clip(path.file_stem().unwrap()) {
				FindClip::One(p) => p != path.strip_prefix(&*CLIP_PATH).unwrap().with_extension(""),
				_ => true,
			}
		})
		.for_each(|s| {
			warn!(
				"Clip {:?} does not get found searching for the exact name",
				s.path()
			)
		});
}

/// Try to find a clip based on the search `loc`.
///
/// If the clip is recognized as a URL, it leaves it as is.
///
/// The actual search is done by searching for the lowest levenshtein distance.
/// Ties are broken by using whichever clip has the longest match, followed by
/// whichever clip has the shortest path, including the directory. In the case
/// there is still a tie [`FindClip::Multiple`] is returned.
///
/// As specified by [`triple_accel::levenshtein::levenshtein_search`], half the
/// bytes of the search have to be found in the clip, or else it is possible
/// for [`FindClip::None`] to be returned.
pub fn find_clip(loc: &OsStr) -> FindClip {
	if URL.is_match(&loc.to_string_lossy()) {
		return FindClip::One(loc.to_owned());
	}

	let top_two = WalkDir::new(&*CLIP_PATH)
		.into_iter()
		.filter_map(|f| f.ok())
		.filter(|f| f.file_type().is_file())
		// calculate the levenshtein distance of each file
		// break ties by prioritizing longest length of match
		// followed by shortest length of clip path
		.filter_map(|f| {
			let path = f.path().to_string_lossy();

			let leven = triple_accel::levenshtein::levenshtein_search(
				loc.to_string_lossy().as_bytes(),
				path.as_bytes(),
			)
			.next()?;

			Some(OrdKey {
				key: (leven.k, -((leven.end - leven.start) as isize), path.len()),
				value: f,
			})
		})
		.k_smallest(2)
		.collect_vec();

	debug!("Found the follwing top two clips: {:?}", top_two);

	if top_two.is_empty() {
		FindClip::None
	} else if top_two.len() > 1 && top_two[0].key == top_two[1].key {
		FindClip::Multiple(
			top_two[0]
				.value
				.path()
				.strip_prefix(&*CLIP_PATH)
				.unwrap()
				.with_extension("")
				.into(),
			top_two[1]
				.value
				.path()
				.strip_prefix(&*CLIP_PATH)
				.unwrap()
				.with_extension("")
				.into(),
		)
	} else {
		FindClip::One(
			top_two[0]
				.value
				.path()
				.strip_prefix(&*CLIP_PATH)
				.unwrap()
				.with_extension("")
				.into(),
		)
	}
}

pub fn get_clip(loc: &OsStr) -> Option<OsString> {
	if URL.is_match(&loc.to_string_lossy()) {
		return Some(loc.to_os_string());
	}

	let mut play_path = Path::new(loc).to_path_buf();

	for ext in &["mp3", "wav"] {
		play_path.set_extension(ext);

		if valid_clip(&play_path) {
			return Some(play_path.into());
		}
	}

	None
}

/// Verify that the clip exists within the clip path directory.
pub fn valid_clip(path: &Path) -> bool {
	sandboxed_join(&CLIP_PATH, path).is_some()
}
