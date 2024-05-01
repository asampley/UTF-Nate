//! Some shared utilities.
//!
//! This includes some functions to make sure files that are accessed are only
//! contained within a folder to prevent people from going up directories
//! ([`sandboxed_join`]), and some functions to help responding to commands.

mod respond;

pub use respond::{Respond, Response};

use songbird::input::AuxMetadata;
use tracing::error;

use serde::Deserialize;

use serenity::prelude::{TypeMap, TypeMapKey};

use thiserror::Error;

use std::fmt;
use std::fmt::Debug;
use std::path::{Component, Path, PathBuf};
use std::time::Duration;

pub type Data = ();
pub type Command = poise::Command<Data, CommandError>;
pub type CommandError = Box<dyn std::error::Error + Send + Sync>;
pub type CommandResult = Result<(), CommandError>;
pub type Context<'a> = poise::Context<'a, Data, CommandError>;
pub type FrameworkError<'a> = poise::FrameworkError<'a, Data, CommandError>;
pub type Framework = poise::Framework<Data, CommandError>;

#[derive(Debug, Error)]
pub enum UtilError {
	Serenity(#[from] serenity::Error),
	Songbird(#[from] songbird::input::core::errors::Error),
}

impl fmt::Display for UtilError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Debug::fmt(self, f)
	}
}

/// Errors that can occur when reading a toml file.
#[derive(Debug, Error)]
pub enum TomlFileError {
	#[error("unable to parse toml: {0}")]
	TomlError(#[from] toml::de::Error),
	#[error("unable to read file: {0}")]
	IoError(#[from] std::io::Error),
}

/// Confirm that a path relative to the `sandbox` does not move up outside of
/// the sandbox. It additionally checks that the path exists.
pub fn sandboxed_join(sandbox: &Path, path: impl AsRef<Path>) -> Option<PathBuf> {
	match sandbox.canonicalize() {
		Ok(sandbox) => {
			// check for any illegal components
			let illegal_components = path
				.as_ref()
				.components()
				// normal or current dir okay
				.any(|c| !matches!(c, Component::Normal(_) | Component::CurDir));

			// return false if any components are illegal
			// prevents scanning the directory structure
			if illegal_components {
				return None;
			}

			// because we've checked above for no backtracking or root, we can join and check
			// existence
			let joined = sandbox.join(path);

			if joined.exists() {
				Some(joined)
			} else {
				None
			}
		}
		Err(_) => None,
	}
}

/// Log an error if `result` is an error. This is useful for checking if
/// sending a message was successful.
pub fn check_msg<T, E>(result: Result<T, E>)
where
	E: Debug,
{
	if let Err(reason) = result {
		error!("Error sending message: {:?}", reason);
	}
}

/// Trait which combines get and expect, panicking if the get function fails.
///
/// This is useful for when the get function should never fail, but could if
/// something was inconfigured correctly.
#[allow(dead_code)]
pub trait GetExpect {
	fn get_expect<T: TypeMapKey>(&self) -> &<T as TypeMapKey>::Value;

	fn get_mut_expect<T: TypeMapKey>(&mut self) -> &mut <T as TypeMapKey>::Value;

	fn clone_expect<T>(&self) -> <T as TypeMapKey>::Value
	where
		T: TypeMapKey,
		<T as TypeMapKey>::Value: Clone,
	{
		self.get_expect::<T>().clone()
	}
}

impl GetExpect for TypeMap {
	fn get_expect<T: TypeMapKey>(&self) -> &<T as TypeMapKey>::Value {
		self.get::<T>()
			.unwrap_or_else(|| panic!("Expected {} in TypeMap", std::any::type_name::<T>()))
	}

	fn get_mut_expect<T: TypeMapKey>(&mut self) -> &mut <T as TypeMapKey>::Value {
		self.get_mut::<T>()
			.unwrap_or_else(|| panic!("Expected {} in TypeMap", std::any::type_name::<T>()))
	}
}

/// Read and parse a toml file.
pub fn read_toml<T, P>(path: P) -> Result<T, TomlFileError>
where
	T: for<'de> Deserialize<'de>,
	P: AsRef<Path>,
{
	Ok(toml::from_str(&std::fs::read_to_string(path.as_ref())?)?)
}

/// Format a duration as minutes and seconds, as well as hours if applicable
pub fn write_duration(write: &mut impl std::fmt::Write, duration: Duration) -> std::fmt::Result {
	let total_seconds = duration.as_secs();
	let s = total_seconds % 60;
	let m = (total_seconds / 60) % 60;
	let h = total_seconds / 3600;

	if h > 0 {
		write!(write, "{h}:{m:>02}:{s:>02}")
	} else {
		write!(write, "{m}:{s:>02}")
	}
}

/// Format track metadata in markdown, as the title (potentially linked), and
/// the duration.
///
/// If the title is unavailable it is named "Unknown".
///
/// All other fields if unavailable are omitted.
pub fn write_track(write: &mut impl std::fmt::Write, meta: &AuxMetadata) -> std::fmt::Result {
	let title = meta.title.as_deref().unwrap_or("Unknown");

	match &meta.source_url {
		Some(url) => write!(write, " [{title}]({url})")?,
		None => write!(write, " {title}")?,
	}

	if let Some(duration) = meta.duration {
		write!(write, " (")?;
		write_duration(write, duration)?;
		write!(write, ")")?;
	};

	Ok(())
}

#[cfg(test)]
mod test {
	use std::{assert_matches::assert_matches, path::PathBuf};

	use tap::TapFallible;

	use tracing::error;
	use walkdir::{DirEntry, WalkDir};

	use super::sandboxed_join;

	use crate::audio::CLIP_PATH;

	fn test_files() -> impl Iterator<Item = DirEntry> {
		WalkDir::new(&*CLIP_PATH)
			.into_iter()
			.filter_map(|f| f.tap_err(|e| error!("{:?}", e)).ok())
			.filter(|f| f.file_type().is_file())
	}

	#[test]
	fn test_case_not_empty() {
		assert!(test_files().any(|_| true));
	}

	#[test]
	fn sandboxed_fails_bad_path() {
		for entry in test_files() {
			let relative = entry
				.path()
				.strip_prefix(&*CLIP_PATH)
				.expect("Unexpected prefix");
			let non_existant = relative.with_extension("doesnotexist");

			assert_eq!(None, sandboxed_join(&CLIP_PATH, &non_existant));
		}
	}

	#[test]
	fn sandboxed_gets_relative() {
		for entry in test_files() {
			let relative = entry
				.path()
				.strip_prefix(&*CLIP_PATH)
				.expect("Unexpected prefix");

			assert_matches!(sandboxed_join(&CLIP_PATH, relative), Some(_));
		}
	}

	#[test]
	fn sandboxed_fails_moving_up() {
		for entry in test_files() {
			let relative = entry
				.path()
				.strip_prefix(&*CLIP_PATH)
				.expect("Unexpected prefix");
			let relative = [
				"..".as_ref(),
				CLIP_PATH.components().last().unwrap().as_ref(),
				relative,
			]
			.iter()
			.collect::<PathBuf>();

			assert!((CLIP_PATH.join(&relative)).exists());

			assert_eq!(None, sandboxed_join(&CLIP_PATH, relative));
		}
	}

	#[test]
	fn sandboxed_fails_absolute() {
		for entry in test_files() {
			let absolute = entry.path();

			assert!(absolute.exists());

			assert_matches!(sandboxed_join(&CLIP_PATH, absolute), None);
		}
	}
}
