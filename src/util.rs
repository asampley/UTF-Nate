use serenity::async_trait;
use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::model::id::GuildId;
use serenity::model::prelude::Guild;
use serenity::prelude::{ TypeMap, TypeMapKey };

use std::fmt;
use std::path::Path;

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
			msg.channel_id.say(&ctx.http, say.as_ref()).await?;
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
			msg.channel_id.say(&ctx.http, say.as_ref()).await?;
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
				msg.channel_id.say(&ctx.http, say.as_ref()).await?;
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
				msg.channel_id.say(&ctx.http, say.as_ref()).await?;
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
			msg.channel_id.say(&ctx.http, say.as_ref()).await?;
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
			msg.channel_id.say(&ctx.http, say.as_ref()).await?;
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
				msg.channel_id.say(&ctx.http, say.as_ref()).await?;
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
				msg.channel_id.say(&ctx.http, say.as_ref()).await?;
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
		eprintln!("Error sending message: {:?}", reason);
	}
}

pub async fn msg_guild_or_say(ctx: &Context, msg: &Message) -> Result<Guild, UtilError> {
	msg.guild(&ctx.cache).await.or_err_say(ctx, msg, "This command is only available in guilds").await
}

pub async fn msg_guild_id_or_say(ctx: &Context, msg: &Message) -> Result<GuildId, UtilError> {
	msg.guild_id.or_err_say(ctx, msg, "This command is only available in guilds").await
}

#[async_trait]
pub trait GetExpect {
	fn get_expect<T: TypeMapKey>(&self) -> &<T as TypeMapKey>::Value;

	fn get_mut_expect<T: TypeMapKey>(&mut self) -> &mut <T as TypeMapKey>::Value;

	fn clone_expect<T>(&self) -> <T as TypeMapKey>::Value
	where
		T: TypeMapKey,
		<T as TypeMapKey>::Value: Clone
	{
		self.get_expect::<T>().clone()
	}
}

impl GetExpect for TypeMap {
	fn get_expect<T: TypeMapKey>(&self) -> &<T as TypeMapKey>::Value {
		self.get::<T>()
			.expect(&format!("Expected {} in TypeMap", std::any::type_name::<T>()))
	}
	
	fn get_mut_expect<T: TypeMapKey>(&mut self) -> &mut <T as TypeMapKey>::Value {
		self.get_mut::<T>()
			.expect(&format!("Expected {} in TypeMap", std::any::type_name::<T>()))
	}
}
