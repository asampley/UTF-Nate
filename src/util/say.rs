use serenity::async_trait;
use serenity::client::Context;
use serenity::model::channel::Message;

use crate::util::{Respond, UtilError};

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
			msg.respond_ok(ctx, say.as_ref().to_string().into()).await?;
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
			msg.respond_ok(ctx, say.as_ref().to_string().into()).await?;
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
				msg.respond_err(ctx, say.as_ref().to_string().into()).await?;
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
				msg.respond_err(ctx, say.as_ref().to_string().into()).await?;
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
			msg.respond_ok(ctx, say.as_ref().to_string().into()).await?;
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
			msg.respond_ok(ctx, say.as_ref().to_string().into()).await?;
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
				msg.respond_err(ctx, say.as_ref().to_string().into()).await?;
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
				msg.respond_err(ctx, say.as_ref().to_string().into()).await?;
				Err(UtilError::Other(say.to_string()))
			}
		}
	}
}
