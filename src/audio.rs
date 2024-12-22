//! Fetch Spotify, YouTube, online audio files, and locally stored clips for
//! playing.
//!
//! Ultimately all audio sources that are streamed are from youtube, but the
//! metadata from Spotify is parsed in order to create a search on youtube
//! which will likely return the source being searched for.
//!
//! Clip searches are done using levenshtein distance in order to fuzzily
//! match making it easier to use without knowing exact clip names.

use futures::TryStreamExt;

use itertools::Itertools;

use songbird::error::TrackResult;
use songbird::input::{AudioStream, AudioStreamError, AuxMetadata, Compose};
use songbird::tracks::PlayMode;
use songbird::Call;

use symphonia::core::io::MediaSource;

use tracing::{debug, error, info, warn};

use rand::seq::IteratorRandom;

use regex::Regex;

use reqwest::Url;

use serenity::async_trait;

use songbird::input::Input;

use thiserror::Error;

use walkdir::WalkDir;

use std::borrow::Cow;
use std::cmp::min;
use std::collections::HashSet;
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use crate::data::{ArcRw, Keys};
use crate::parser::Selection;
use crate::util::*;
use crate::youtube::{self, compose_yt_search};
use crate::RESOURCE_PATH;
use crate::{spotify, REQWEST_CLIENT};

/// Path to shared directory for clips.
pub static CLIP_PATH: LazyLock<PathBuf> = LazyLock::new(|| RESOURCE_PATH.join("clips/"));

/// Regular expression which matches valid http or https urls.
static URL: LazyLock<Regex> = LazyLock::new(|| Regex::new("^https?://").unwrap());

/// Regular expression which matches the host portion of a url if the host is youtube.
static YOUTUBE_HOST: LazyLock<Regex> =
	LazyLock::new(|| Regex::new("^([^.]*\\.)?(youtube\\.com|youtu.be)").unwrap());

/// Regular expression which matches the host portion of a url if the host is spotify.
static SPOTIFY_HOST: LazyLock<Regex> =
	LazyLock::new(|| Regex::new("^open\\.spotify\\.com").unwrap());

/// Enum for the two styles of audio source.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlayStyle {
	/// Remote streamed audio sources that are queued.
	Play,
	/// Local audio clips that are played immediately.
	Clip,
}

/// Enum for places to search when a URL is not provided.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SearchSource {
	/// Search on youtube.
	Youtube,
	/// Search through local clip files.
	Local,
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

pub struct ComposeWithMetadata<C> {
	pub compose: C,
	pub aux_metadata: AuxMetadata,
}

impl<C> ComposeWithMetadata<C> {
	pub fn new(compose: C, aux_metadata: AuxMetadata) -> Self {
		Self {
			compose,
			aux_metadata,
		}
	}
}

#[async_trait]
impl<C: Compose> Compose for ComposeWithMetadata<C> {
	fn create(&mut self) -> Result<AudioStream<Box<dyn MediaSource>>, AudioStreamError> {
		self.compose.create()
	}

	async fn create_async(
		&mut self,
	) -> Result<AudioStream<Box<dyn MediaSource>>, AudioStreamError> {
		self.compose.create_async().await
	}

	fn should_create_async(&self) -> bool {
		self.compose.should_create_async()
	}

	async fn aux_metadata(&mut self) -> Result<AuxMetadata, AudioStreamError> {
		Ok(self.aux_metadata.clone())
	}
}

impl<C: Compose + 'static> From<ComposeWithMetadata<C>> for Input {
	fn from(compose: ComposeWithMetadata<C>) -> Self {
		Self::Lazy(Box::new(compose))
	}
}

/// Error type for all audio errors in fetching clips or playing clips.
#[derive(Debug, Error)]
pub enum AudioError {
	/// Pass through to a songbird error.
	#[error("encountered symphonia error: {0}")]
	Symphonia(#[from] songbird::input::core::errors::Error),

	#[error("encountered songbird error: {0}")]
	Songbird(#[from] songbird::input::AudioStreamError),

	#[error("error while retrieving metadata: {0}")]
	Metadata(#[from] songbird::input::AuxMetadataError),

	#[error("error while fetching from youtube api: {0}")]
	YoutubeApi(youtube::Error),

	/// Error indicating the context does not allow playlists.
	#[error("playlists are not allowed in this context")]
	PlaylistNotAllowed,

	/// Generic error when using spotify.
	#[error("error using the spotify api")]
	Spotify,

	/// Generic error when fetching a youtube playlist.
	#[error("error reading youtube api key")]
	YoutubeApiKey,

	/// Error indicating that the url that was supplied cannot be used to get clips.
	#[error("unsupported url")]
	UnsupportedUrl,

	/// Error indicating no matching clips were found.
	#[error("no clips matched")]
	NotFound,
}

/// Contains a few details for a source for display. This could be a single source
/// or a playlist, but either way should have a title and url, if possible.
pub struct SourceInfo {
	/// The title that can be used as a good display string.
	pub title: Option<String>,

	/// The url which actually directs to the resource.
	pub url: Option<String>,

	/// The number of actual sources, if a playlist.
	pub count: usize,

	/// duration of all sources added up
	pub duration: Option<std::time::Duration>,

	/// Iterator for the inputs
	pub inputs: Box<dyn Iterator<Item = Input> + Send + Sync>,
}

/// Creates audio inputs for [`songbird`], and as each audio inputs is created.
/// Information about the audio inputs is included with an iterator over the
/// inputs.
///
/// If `loc` is a URL, as matched by the [`URL`] regular expression, it will try
/// to match either a youtube or spotify URL, based on the host names in the
/// [`YOUTUBE_HOST`] and [`SPOTIFY_HOST`] regular expressions. Any other URL
/// will be streamed as an audio file, and if that fails, it returns an
/// [`AudioError::UnsupportedUrl`] error.
///
/// If `loc` is not a URL then it will instead do a search on youtube and grab
/// the first match.
///
/// Certain contexts may wish to exclude playlists, so `allow_playlist` can be
/// set to false return an [`AudioError::PlaylistNotAllowed`] instead.
pub async fn get_inputs(
	keys: ArcRw<Keys>,
	loc: &str,
	allow_playlist: bool,
	search_location: Option<SearchSource>,
) -> Result<SourceInfo, AudioError> {
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
				.ok_or(AudioError::YoutubeApiKey)?;

			if path == "/playlist" {
				// youtube playlist
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
					.inspect_err(|e| error!("Error in youtube data api for playlists: {:?}", e))
					.map_err(AudioError::YoutubeApi)?
					.ok_or_else(|| {
						error!("No playlist found");
						AudioError::NotFound
					})?;

				let videos = youtube::playlist_items(&youtube_api, &id)
					.map_ok(|items| {
						youtube::videos(
							&youtube_api,
							items.into_iter().map(|item| item.content_details.video_id),
						)
					})
					// concurrently hit videos api after paged playlist items
					// no gains were seen after 3 concurrent, 4 is just in case
					.try_buffered(4)
					.try_concat()
					.await
					.inspect_err(|e| {
						error!("Error in youtube data api for playlist items: {:?}", e)
					})
					.map_err(AudioError::YoutubeApi)?;

				let count = videos.len();

				Ok(SourceInfo {
					title: Some(playlist.snippet.title),
					url: Some(loc.to_string()),
					count,
					duration: videos
						.iter()
						.map(|video| video.content_details.duration.to_std())
						.try_fold(std::time::Duration::ZERO, |acc, opt| opt.map(|x| acc + x)),
					inputs: Box::new(
						videos
							.into_iter()
							.map(|item| Input::from(ComposeWithMetadata::from(item))),
					),
				})
			} else {
				// single youtube video
				let id = if path == "/watch" {
					url.query_pairs()
						.filter(|(key, _)| key == "v")
						.map(|(_, value)| value)
						.next()
						.ok_or_else(|| {
							debug!("Missing \"v\" in query parameters: {}", url);
							AudioError::UnsupportedUrl
						})?
				} else {
					Cow::Borrowed(&path[1..])
				};

				let video = youtube::video(&youtube_api, &id)
					.await
					.inspect_err(|e| error!("Youtube video error: {:?}", e))
					.map_err(AudioError::YoutubeApi)?
					.ok_or_else(|| {
						error!("No video found with id {:?}", id);
						AudioError::NotFound
					})?;

				let duration = video.content_details.duration.to_std();
				let title = video.snippet.title.clone();

				let compose = ComposeWithMetadata::from(video);

				Ok(SourceInfo {
					title: Some(title),
					url: Some(loc.to_string()),
					count: 1,
					duration,
					inputs: Box::new(std::iter::once_with(|| compose.into())),
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
					// spotify single song
					let track_id = path_segments.next().ok_or(AudioError::UnsupportedUrl)?;

					let track = spotify::track(&token, track_id)
						.await
						.inspect_err(|e| error!("Error reading spotify track: {:?}", e))
						.map_err(|_| AudioError::Spotify)?;

					let compose = ComposeWithMetadata::from(&track);

					Ok(SourceInfo {
						title: Some(track.name),
						url: Some(loc.to_string()),
						count: 1,
						duration: None,
						inputs: Box::new(std::iter::once_with(|| compose.into())),
					})
				}
				"playlist" => {
					// spotify playlist
					if !allow_playlist {
						return Err(AudioError::PlaylistNotAllowed);
					}

					let playlist_id = path_segments.next().ok_or(AudioError::UnsupportedUrl)?;

					// First try the api
					let playlist = match spotify::playlist(&token, playlist_id).await {
						Ok(v) => v,
						Err(e) => {
							// Fall back to parsing an embed in case spotify does not display that
							// playlist on the api
							spotify::scrape::playlist(playlist_id)
								.await
								.inspect_err(|e_scrape| {
									error!("Error reading spotify playlist: {:?}", e);
									error!("Error scraping spotify playlist: {:?}", e_scrape);
								})
								.map_err(|_| AudioError::Spotify)?
						}
					};

					let items = spotify::into_stream(&token, playlist.tracks)
						.map_ok(|playlist_tracks| playlist_tracks.items)
						.try_concat()
						.await
						.inspect_err(|e| {
							error!("Error in spotify data api for playlist items: {:?}", e)
						})
						.map_err(|_| AudioError::Spotify)?;

					Ok(SourceInfo {
						title: Some(playlist.name),
						url: Some(loc.to_string()),
						count: items.len(),
						duration: None,
						inputs: Box::new(
							items
								.into_iter()
								.map(|t| Input::from(ComposeWithMetadata::from(&t.track))),
						),
					})
				}
				"album" => {
					// spotify playlist
					if !allow_playlist {
						return Err(AudioError::PlaylistNotAllowed);
					}

					let album_id = path_segments.next().ok_or(AudioError::UnsupportedUrl)?;

					let album = spotify::album(&token, album_id)
						.await
						.inspect_err(|e| error!("Error reading spotify album: {:?}", e))
						.map_err(|_| AudioError::Spotify)?;

					let items = spotify::into_stream(&token, album.tracks)
						.map_ok(|album_tracks| album_tracks.items)
						.try_concat()
						.await
						.inspect_err(|e| {
							error!("Error in spotify data api for album items: {:?}", e)
						})
						.map_err(|_| AudioError::Spotify)?;

					Ok(SourceInfo {
						title: Some(album.name),
						url: Some(loc.to_string()),
						count: items.len(),
						duration: None,
						inputs: Box::new(
							items
								.into_iter()
								.map(|track| Input::from(ComposeWithMetadata::from(&track))),
						),
					})
				}
				_ => Err(AudioError::UnsupportedUrl),
			}
		} else {
			// arbitrary audio file url
			let title = url
				.path_segments()
				.map(|mut p| p.nth_back(0).unwrap().to_string());

			let mut compose = ComposeWithMetadata::new(
				songbird::input::HttpRequest::new(REQWEST_CLIENT.clone(), loc.to_string()),
				AuxMetadata {
					title: title.clone(),
					source_url: Some(loc.to_string()),
					..Default::default()
				},
			);

			let aux_metadata = compose
				.aux_metadata()
				.await
				.inspect_err(|e| error!("Error getting metadata: {:?}", e))?;

			Ok(SourceInfo {
				title,
				url: Some(loc.to_string()),
				count: 1,
				duration: aux_metadata.duration,
				inputs: Box::new(std::iter::once_with(|| compose.into())),
			})
		}
	} else {
		// arbitrary search term
		match search_location {
			Some(SearchSource::Youtube) => {
				let loc_string = loc.to_string();

				let mut compose = compose_yt_search(loc_string);

				let aux_metadata = compose
					.aux_metadata()
					.await
					.inspect_err(|e| error!("Error getting metadata: {:?}", e))?;

				Ok(SourceInfo {
					title: aux_metadata.title,
					url: aux_metadata.source_url,
					count: 1,
					duration: aux_metadata.duration,
					inputs: Box::new(std::iter::once_with(|| compose.into())),
				})
			}
			Some(SearchSource::Local) => {
				let clips = search_clips(loc.as_ref());

				let clip_name = clips
					.into_iter()
					.choose(&mut rand::thread_rng())
					.ok_or(AudioError::NotFound)?;

				let clip = get_clip(&clip_name).ok_or(AudioError::NotFound)?;

				let aux_metadata = AuxMetadata {
					title: Some(clip_name.to_string_lossy().into_owned()),
					..Default::default()
				};

				let compose =
					ComposeWithMetadata::new(songbird::input::File::new(clip), aux_metadata);

				Ok(SourceInfo {
					title: Some(clip_name.to_string_lossy().into_owned()),
					url: None,
					count: 1,
					duration: None,
					inputs: Box::new(std::iter::once_with(|| compose.into())),
				})
			}
			// try to get an exact match on the clip, else fail
			None => {
				let clip = get_clip(loc.as_ref()).ok_or(AudioError::NotFound)?;

				let aux_metadata = AuxMetadata {
					title: Some(loc.to_owned()),
					..Default::default()
				};

				let compose =
					ComposeWithMetadata::new(songbird::input::File::new(clip), aux_metadata);

				Ok(SourceInfo {
					title: Some(loc.to_owned()),
					url: None,
					count: 1,
					duration: None,
					inputs: Box::new(std::iter::once_with(|| compose.into())),
				})
			}
		}
	}
}

#[tracing::instrument(level = "info", ret, skip(call))]
pub async fn move_queue(
	call: &mut Call,
	selection: Selection<usize>,
	position: usize,
) -> TrackResult<usize> {
	let queue = call.queue();

	let Some(current) = queue.current() else {
		return Ok(0);
	};

	let resume = current.get_info().await?.playing == PlayMode::Play;

	if position == 0 {
		queue.pause()?
	}

	let moved = queue.modify_queue(|deque| {
		let selection_iter = selection.into_iter();

		// once accounted for moving, don't move twice
		let moving: HashSet<_> = selection_iter.clone().collect();

		let mut indices = vec![usize::MAX; deque.len()];

		// position can at most be the length of the queue less the size of the selection
		let position = min(deque.len() - moving.len(), position);

		// fill in selection indices first
		let mut dest = position;
		for s in selection_iter {
			if s < deque.len() && indices[s] == usize::MAX {
				indices[s] = dest;
				dest += 1;
			}
		}

		// fill in the rest of the indices
		let mut dest_rest = 0;
		for i in &mut indices {
			// skip to end of selection if we've hit the start
			if dest_rest == position {
				dest_rest = dest;
			}
			// change anything not yet set
			if *i == usize::MAX {
				*i = dest_rest;
				dest_rest += 1;
			}
		}

		// swap element until everything is in order
		// this will terminate because each step puts one
		// more element in the correct place, and it finishes
		// when all elements are in the correct place.
		let mut i = 0;
		while i < indices.len() {
			let goto = indices[i];

			if i == goto {
				i += 1;
			} else {
				deque.swap(i, goto);
				indices.swap(i, goto);
			}
		}

		moving.len()
	});

	if resume {
		queue.resume()?
	};

	info!("Moved tracks {:?}", moved);

	Ok(moved)
}

/// Log a warning if any clips have the same file stem.
///
/// Different extensions are ignored, the comparison is done using
/// [`Path::file_stem`]. The messages are logged using [`warn!()`].
pub fn warn_duplicate_clip_names() {
	WalkDir::new(&*CLIP_PATH)
		.into_iter()
		.filter_map(|f| f.inspect_err(|e| error!("{:?}", e)).ok())
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
		.filter_map(|f| f.inspect_err(|e| error!("{:?}", e)).ok())
		.filter(|f| f.file_type().is_file())
		.filter(|f| {
			let path = f.path();

			match &search_clips(path.file_stem().unwrap())[..] {
				[p] => p != &path.strip_prefix(&*CLIP_PATH).unwrap().with_extension(""),
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
/// If the clip matches a URL, it is just returned.
///
/// If the clip matches a file exactly (excluding extension), that is returned
/// skipping the search.
///
/// The actual search is done by searching for the lowest levenshtein distance.
/// Ties are broken by using whichever clip has the longest match, followed by
/// whichever clip has the shortest path, including the directory.
///
/// As specified by [`triple_accel::levenshtein::levenshtein_search`], half the
/// bytes of the search have to be found in the clip, or else it is possible
/// for no clips to be returned.
#[tracing::instrument(level = "info", ret)]
pub fn search_clips(loc: &OsStr) -> Vec<OsString> {
	if URL.is_match(&loc.to_string_lossy()) {
		return vec![loc.to_owned()];
	}

	// short circuit exact match
	if let Some(path) = get_clip(loc) {
		return vec![path];
	}

	WalkDir::new(&*CLIP_PATH)
		.into_iter()
		.filter_map(|f| f.inspect_err(|e| error!("{:?}", e)).ok())
		.filter(|f| f.file_type().is_file())
		// calculate the levenshtein distance of each file
		// break ties by prioritizing longest length of match
		// followed by shortest length of clip path
		.min_set_by_key(|f| {
			let path = f.path().to_string_lossy();

			let leven = triple_accel::levenshtein::levenshtein_search(
				loc.to_string_lossy().as_bytes(),
				path.as_bytes(),
			)
			.next();

			match leven {
				Some(leven) => (leven.k, -((leven.end - leven.start) as isize), path.len()),
				None => (u32::MAX, isize::MAX, usize::MAX),
			}
		})
		.into_iter()
		.map(|f| {
			f.path()
				.strip_prefix(&*CLIP_PATH)
				.unwrap()
				.with_extension("")
				.into()
		})
		.collect_vec()
}

pub fn get_clip(loc: &OsStr) -> Option<OsString> {
	if URL.is_match(&loc.to_string_lossy()) {
		return Some(loc.to_os_string());
	}

	let mut play_path = Path::new(loc).to_path_buf();

	for ext in &["mp3", "wav"] {
		play_path.set_extension(ext);

		if valid_clip(&play_path) {
			return Some(CLIP_PATH.join(play_path).into());
		}
	}

	None
}

pub fn clip_iter() -> impl Iterator<Item = OsString> {
	WalkDir::new(&*CLIP_PATH)
		.into_iter()
		.filter_map(|f| f.inspect_err(|e| error!("{:?}", e)).ok())
		.filter(|f| f.file_type().is_file())
		.map(|f| f.path().strip_prefix(&*CLIP_PATH).unwrap().into())
}

/// Verify that the clip exists within the clip path directory.
pub fn valid_clip(path: &Path) -> bool {
	sandboxed_join(&CLIP_PATH, path).is_some()
}

#[cfg(test)]
mod tests {
	use super::*;

	const URLS: &[&str] = &[
		// youtube single video
		"https://www.youtube.com/watch?v=k2mFvwDTTt0",
		// youtube playlist
		"https://www.youtube.com/playlist?list=PLucOLpdAYaKW1IYuo84R4qIskTfj-ECDp",
		// spotify single track
		"https://open.spotify.com/track/009bpReJuXgCv8G2MkJ5Y1",
		// spotify album
		"https://open.spotify.com/album/0G2RxSCixG5Nl6jpjwiw2g",
		// spotify playlist
		"https://open.spotify.com/playlist/2O18dCV9uoGTyxN5HLJkTo",
	];

	// test to make sure inputs equal count
	#[tokio::test]
	async fn play_sources_count() {
		let keys: ArcRw<_> = std::sync::Arc::new(read_toml::<Keys, _>("keys.toml").unwrap().into());

		for url in URLS {
			let sources = get_inputs(keys.clone(), url, true, None).await.unwrap();

			assert_eq!(sources.count, sources.inputs.count())
		}
	}
}
