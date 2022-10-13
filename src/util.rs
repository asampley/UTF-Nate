//! Some shared utilities.
//!
//! This includes some functions to make sure files that are accessed are only
//! contained within a folder to prevent people from going up directories
//! ([`sandboxed_exists`]), and some functions to help responding to commands.

mod ord_key;
mod respond;

pub use ord_key::OrdKey;
pub use respond::{Log, Respond, Response};

use tracing::error;

use serde::Deserialize;

use serenity::async_trait;
use serenity::prelude::{TypeMap, TypeMapKey};

use thiserror::Error;

use std::fmt;
use std::path::{Component, Path};

type Data = ();
pub type Command = poise::Command<Data, CommandError>;
pub type CommandError = Box<dyn std::error::Error + Send + Sync>;
pub type CommandResult = Result<(), CommandError>;
pub type Context<'a> = poise::Context<'a, Data, CommandError>;
pub type FrameworkError<'a> = poise::FrameworkError<'a, Data, CommandError>;

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
pub fn sandboxed_exists(sandbox: &Path, path: &Path) -> bool {
	match sandbox.canonicalize() {
		Ok(sandbox) => {
			// check for any illegal components
			let illegal_components = path
				.components()
				// normal or current dir okay
				.map(|c| !matches!(c, Component::Normal(_) | Component::CurDir))
				.any(|illegal| illegal);

			// return false if any components are illegal
			// prevents scanning the directory structure
			if illegal_components {
				return false;
			}

			// return false if the canonicalized path does not have the sandbox as a parent
			// prevents accessing files outside of the parent
			match path.canonicalize() {
				Ok(path) => path.ancestors().any(|d| d == sandbox) && path.exists(),
				Err(_) => false,
			}
		}
		Err(_) => false,
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
