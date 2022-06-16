mod ord_key;
mod respond;

pub use ord_key::OrdKey;
pub use respond::{Log, Respond, Response};

use log::error;

use serde_json::Value;

use serenity::async_trait;
use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::model::interactions::application_command::{
	ApplicationCommandInteraction, ApplicationCommandInteractionDataOption,
};
use serenity::prelude::{SerenityError, TypeMap, TypeMapKey};

use std::fmt;
use std::path::Path;

pub fn get_option<'a>(
	options: &'a Vec<ApplicationCommandInteractionDataOption>,
	name: &str,
) -> Option<&'a ApplicationCommandInteractionDataOption> {
	options.iter().find_map(|option| {
		if option.name == name {
			Some(option)
		} else {
			None
		}
	})
}

pub fn get_option_value<'a>(
	options: &'a Vec<ApplicationCommandInteractionDataOption>,
	name: &str,
) -> Option<&'a Value> {
	get_option(options, name).and_then(|o| o.value.as_ref())
}

pub async fn get_option_string<'a>(
	ctx: &Context,
	interaction: &'a ApplicationCommandInteraction,
	options: &'a Vec<ApplicationCommandInteractionDataOption>,
	name: &str,
) -> Result<Option<&'a str>, SerenityError> {
	match get_option_value(options, name) {
		Some(Value::String(s)) => Ok(Some(s.as_str())),
		None => Ok(None),
		Some(v) => {
			error!("Error in interaction expecting string argument");
			interaction
				.respond_err(&ctx, &"Internal bot error".into())
				.await
				.or_log();

			Err(SerenityError::Decode("Expected a string", v.clone()))
		}
	}
}

pub async fn get_option_f32<'a>(
	ctx: &Context,
	interaction: &'a ApplicationCommandInteraction,
	options: &'a Vec<ApplicationCommandInteractionDataOption>,
	name: &str,
) -> Result<Option<f32>, SerenityError> {
	match get_option_value(options, name) {
		Some(Value::Number(n)) => Ok(n.as_f64().map(|v| v as f32)),
		None => Ok(None),
		Some(v) => {
			error!("Error in interaction expecting float argument");
			interaction
				.respond_err(&ctx, &"Internal bot error".into())
				.await
				.or_log();

			Err(SerenityError::Decode("Expected a number", v.clone()))
		}
	}
}

pub async fn get_option_usize<'a>(
	ctx: &Context,
	interaction: &'a ApplicationCommandInteraction,
	options: &'a Vec<ApplicationCommandInteractionDataOption>,
	name: &str,
) -> Result<Option<usize>, SerenityError> {
	match get_option_value(options, name) {
		Some(Value::Number(n)) => Ok(n.as_u64().map(|v| v as usize)),
		None => Ok(None),
		Some(v) => {
			error!("Error in interaction expecting float argument");
			interaction
				.respond_err(&ctx, &"Internal bot error".into())
				.await
				.or_log();

			Err(SerenityError::Decode("Expected a number", v.clone()))
		}
	}
}

#[derive(Debug)]
pub enum UtilError {
	Serenity(serenity::Error),
	Songbird(songbird::input::error::Error),
}

impl From<songbird::input::error::Error> for UtilError {
	fn from(e: songbird::input::error::Error) -> Self {
		Self::Songbird(e)
	}
}

impl From<serenity::Error> for UtilError {
	fn from(e: serenity::Error) -> Self {
		Self::Serenity(e)
	}
}

impl fmt::Display for UtilError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Debug::fmt(self, f)
	}
}

impl std::error::Error for UtilError {}

#[derive(Debug)]
pub enum TomlFileError {
	TomlError(toml::de::Error),
	IoError(std::io::Error),
}

impl From<toml::de::Error> for TomlFileError {
	fn from(e: toml::de::Error) -> Self {
		Self::TomlError(e)
	}
}

impl From<std::io::Error> for TomlFileError {
	fn from(e: std::io::Error) -> Self {
		Self::IoError(e)
	}
}

impl fmt::Display for TomlFileError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Debug::fmt(self, f)
	}
}

impl std::error::Error for TomlFileError {}

pub fn sandboxed_exists(sandbox: &Path, path: &Path) -> bool {
	match sandbox.canonicalize() {
		Ok(sandbox) => match path.canonicalize() {
			Ok(path) => path.ancestors().any(|d| d == sandbox) && path.exists(),
			Err(_) => false,
		},
		Err(_) => false,
	}
}

pub fn check_msg(result: serenity::Result<Message>) {
	if let Err(reason) = result {
		error!("Error sending message: {:?}", reason);
	}
}

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
		self.get::<T>().expect(&format!(
			"Expected {} in TypeMap",
			std::any::type_name::<T>()
		))
	}

	fn get_mut_expect<T: TypeMapKey>(&mut self) -> &mut <T as TypeMapKey>::Value {
		self.get_mut::<T>().expect(&format!(
			"Expected {} in TypeMap",
			std::any::type_name::<T>()
		))
	}
}
