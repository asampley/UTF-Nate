use log::error;

use serenity::async_trait;
use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::model::interactions::application_command::ApplicationCommandInteraction;
use serenity::prelude::{TypeMap, TypeMapKey};

use std::fmt;
use std::path::Path;

#[macro_use]
macro_rules! unwrap_or_ret {
	( $e:expr, $f:expr ) => {
		match $e {
			Some(x) => x,
			None => return $f,
		}
	};
}

#[derive(Debug)]
pub enum UtilError {
	Serenity(serenity::Error),
	Songbird(songbird::input::error::Error),
	Other(String),
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
pub enum JsonFileError {
	JsonError(serde_json::Error),
	IoError(std::io::Error),
}

impl From<serde_json::Error> for JsonFileError {
	fn from(e: serde_json::Error) -> Self {
		Self::JsonError(e)
	}
}

impl From<std::io::Error> for JsonFileError {
	fn from(e: std::io::Error) -> Self {
		Self::IoError(e)
	}
}

impl fmt::Display for JsonFileError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Debug::fmt(self, f)
	}
}

impl std::error::Error for JsonFileError {}

#[async_trait]
pub trait Respond {
	type Value;

	async fn respond_str<T>(&self, ctx: &Context, text: T) -> serenity::Result<Self::Value>
	where
		T: Send + Sync + AsRef<str>;
}

#[async_trait]
impl Respond for Message {
	type Value = Message;

	async fn respond_str<T>(&self, ctx: &Context, text: T) -> serenity::Result<Self>
	where
		T: Send + Sync + AsRef<str>,
	{
		self.channel_id
			.send_message(&ctx.http, |message| {
				message.embed(|embed| embed.description(text.as_ref()))
			})
			.await
	}
}

#[async_trait]
impl Respond for ApplicationCommandInteraction {
	type Value = ();

	async fn respond_str<T>(&self, ctx: &Context, text: T) -> serenity::Result<()>
	where
		T: Send + Sync + AsRef<str>,
	{
		self.create_interaction_response(&ctx.http, |response| {
			response.interaction_response_data(|data| {
				data.create_embed(|embed| embed.description(text.as_ref()))
			})
		})
		.await
	}
}

#[async_trait]
pub trait Say: Sized {
	type OrUnwrap;
	type AndUnwrap;

	async fn and_say<'a>(
		self,
		ctx: &'a Context,
		msg: &'a Message,
		say: impl AsRef<str> + Send + Sync + 'static,
	) -> Result<Self, serenity::Error>;

	async fn or_say<'a>(
		self,
		ctx: &'a Context,
		msg: &'a Message,
		say: impl AsRef<str> + Send + Sync + 'static,
	) -> Result<Self, serenity::Error>;

	async fn and_err_say<'a>(
		self,
		ctx: &'a Context,
		msg: &'a Message,
		say: impl AsRef<str> + ToString + Send + Sync + 'static,
	) -> Result<Self::AndUnwrap, UtilError>;

	async fn or_err_say<'a>(
		self,
		ctx: &'a Context,
		msg: &'a Message,
		say: impl AsRef<str> + ToString + Send + Sync + 'static,
	) -> Result<Self::OrUnwrap, UtilError>;
}

#[async_trait]
impl<T: Send> Say for Option<T> {
	type OrUnwrap = T;
	type AndUnwrap = ();

	async fn and_say<'a>(
		self,
		ctx: &'a Context,
		msg: &'a Message,
		say: impl AsRef<str> + Send + Sync + 'static,
	) -> Result<Self, serenity::Error> {
		if self.is_some() {
			msg.respond_str(ctx, say.as_ref()).await?;
		}

		Ok(self)
	}

	async fn or_say<'a>(
		self,
		ctx: &'a Context,
		msg: &'a Message,
		say: impl AsRef<str> + Send + Sync + 'static,
	) -> Result<Self, serenity::Error> {
		if self.is_none() {
			msg.respond_str(ctx, say.as_ref()).await?;
		}

		Ok(self)
	}

	async fn and_err_say<'a>(
		self,
		ctx: &'a Context,
		msg: &'a Message,
		say: impl AsRef<str> + ToString + Send + Sync + 'static,
	) -> Result<Self::AndUnwrap, UtilError> {
		match self {
			Some(_) => {
				msg.respond_str(ctx, say.as_ref()).await?;
				Err(UtilError::Other(say.to_string()))
			}
			None => Ok(()),
		}
	}

	async fn or_err_say<'a>(
		self,
		ctx: &'a Context,
		msg: &'a Message,
		say: impl AsRef<str> + ToString + Send + Sync + 'static,
	) -> Result<Self::OrUnwrap, UtilError> {
		match self {
			Some(v) => Ok(v),
			None => {
				msg.respond_str(ctx, say.as_ref()).await?;
				Err(UtilError::Other(say.to_string()))
			}
		}
	}
}

#[async_trait]
impl<T: Send, E: Send> Say for Result<T, E> {
	type OrUnwrap = T;
	type AndUnwrap = E;

	async fn and_say<'a>(
		self,
		ctx: &'a Context,
		msg: &'a Message,
		say: impl AsRef<str> + Send + Sync + 'static,
	) -> Result<Self, serenity::Error> {
		if self.is_ok() {
			msg.respond_str(ctx, say.as_ref()).await?;
		}

		Ok(self)
	}

	async fn or_say<'a>(
		self,
		ctx: &'a Context,
		msg: &'a Message,
		say: impl AsRef<str> + Send + Sync + 'static,
	) -> Result<Self, serenity::Error> {
		if self.is_err() {
			msg.respond_str(ctx, say.as_ref()).await?;
		}

		Ok(self)
	}

	async fn and_err_say<'a>(
		self,
		ctx: &'a Context,
		msg: &'a Message,
		say: impl AsRef<str> + ToString + Send + Sync + 'static,
	) -> Result<Self::AndUnwrap, UtilError> {
		match self {
			Ok(_) => {
				msg.respond_str(ctx, say.as_ref()).await?;
				Err(UtilError::Other(say.to_string()))
			}
			Err(e) => Ok(e),
		}
	}

	async fn or_err_say<'a>(
		self,
		ctx: &'a Context,
		msg: &'a Message,
		say: impl AsRef<str> + ToString + Send + Sync + 'static,
	) -> Result<Self::OrUnwrap, UtilError> {
		match self {
			Ok(v) => Ok(v),
			Err(_) => {
				msg.respond_str(ctx, say.as_ref()).await?;
				Err(UtilError::Other(say.to_string()))
			}
		}
	}
}

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
