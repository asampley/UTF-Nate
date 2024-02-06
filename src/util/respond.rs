use poise::{Context, CreateReply, ReplyHandle};

use serenity::builder::{CreateEmbed, CreateMessage};
use serenity::http::Http;
use serenity::model::channel::Message;
use serenity::model::colour::Color;
use serenity::model::id::ChannelId;
use serenity::prelude::SerenityError;

use std::fmt::{Debug, Display};
use std::future::Future;

const OK_COLOR: Color = Color::from_rgb(127, 255, 127);
const ERR_COLOR: Color = Color::from_rgb(255, 127, 127);

#[derive(Clone, Debug)]
pub struct Response {
	pub text: String,
}

impl Display for Response {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		Display::fmt(&self.text, f)
	}
}

impl From<String> for Response {
	fn from(text: String) -> Self {
		Self { text }
	}
}

impl From<&str> for Response {
	fn from(text: &str) -> Self {
		Self {
			text: text.to_string(),
		}
	}
}

impl Response {
	fn embed_color(self, color: Color, create: CreateEmbed) -> CreateEmbed {
		// TODO see if this constant is defined somewhere
		if self.text.len() <= 4096 {
			create.color(color).description(self.text)
		} else {
			create
				.color(ERR_COLOR)
				.description("The response was too long and cannot be displayed")
		}
	}

	fn embed_err(self, create: CreateEmbed) -> CreateEmbed {
		self.embed_color(ERR_COLOR, create)
	}

	fn embed_ok(self, create: CreateEmbed) -> CreateEmbed {
		self.embed_color(OK_COLOR, create)
	}

	fn embed(result: Result<Response, Response>, create: CreateEmbed) -> CreateEmbed {
		match result {
			Ok(response) => response.embed_ok(create),
			Err(response) => response.embed_err(create),
		}
	}
}

pub trait Respond: Sync {
	type Value;
	type Error: Debug;

	fn respond(
		&self,
		result: Result<Response, Response>,
	) -> impl Future<Output = Result<Self::Value, Self::Error>> + Send;

	fn respond_ok(
		&self,
		response: Response,
	) -> impl Future<Output = Result<Self::Value, Self::Error>> + Send {
		async move { self.respond(Ok(response)).await }
	}

	fn respond_err(
		&self,
		response: Response,
	) -> impl Future<Output = Result<Self::Value, Self::Error>> + Send {
		async move { self.respond(Err(response)).await }
	}
}

/// allow responding to be bypassed when the response possibility is unsure.
impl<R: Respond + Send + Sync> Respond for Option<R> {
	type Value = Option<R::Value>;
	type Error = R::Error;

	async fn respond(
		&self,
		result: Result<Response, Response>,
	) -> Result<Self::Value, Self::Error> {
		match self {
			Some(r) => r.respond(result).await.map(Some),
			None => Ok(None),
		}
	}
}

impl<H: AsRef<Http> + Send + Sync> Respond for (H, ChannelId) {
	type Value = Message;
	type Error = SerenityError;

	async fn respond(
		&self,
		result: Result<Response, Response>,
	) -> Result<Self::Value, Self::Error> {
		self.0
			.as_ref()
			.send_message(
				self.1,
				vec![],
				&CreateMessage::new().embed(Response::embed(result, CreateEmbed::new())),
			)
			.await
	}
}

impl<'a, U, E> Respond for Context<'a, U, E>
where
	U: Sync,
{
	type Value = ReplyHandle<'a>;
	type Error = SerenityError;

	async fn respond(
		&self,
		result: Result<Response, Response>,
	) -> Result<Self::Value, Self::Error> {
		self.send(CreateReply::default().embed(Response::embed(result, CreateEmbed::new())))
			.await
	}
}
