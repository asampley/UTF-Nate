//! Some shared utilities.
//!
//! This includes some functions to make sure files that are accessed are only
//! contained within a folder to prevent people from going up directories
//! ([`sandboxed_join`]), and some functions to help responding to commands.

mod ord_key;
mod respond;

pub use ord_key::OrdKey;
pub use respond::{Respond, Response};

use tracing::error;

use serde::Deserialize;

use serenity::async_trait;
use serenity::prelude::{TypeMap, TypeMapKey};

use thiserror::Error;

use std::fmt;
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
	Songbird(#[from] songbird::input::error::Error),
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
				.map(|c| !matches!(c, Component::Normal(_) | Component::CurDir))
				.any(|illegal| illegal);

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
pub fn check_msg<T>(result: serenity::Result<T>) {
	if let Err(reason) = result {
		error!("Error sending message: {:?}", reason);
	}
}

/// Trait which combines get and expect, panicking if the get function fails.
///
/// This is useful for when the get function should never fail, but could if
/// something was inconfigured correctly.
#[async_trait]
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
