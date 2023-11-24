use poise::{Context, ReplyHandle};

use serenity::async_trait;
use serenity::builder::CreateEmbed;
use serenity::utils::Color;

use std::fmt::Debug;

const OK_COLOR: Color = Color::from_rgb(127, 255, 127);
const ERR_COLOR: Color = Color::from_rgb(255, 127, 127);

#[derive(Debug)]
pub struct Response {
	pub text: String,
}

impl std::fmt::Display for Response {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(&self.text, f)
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
	fn embed_color<'a>(&self, color: Color, create: &'a mut CreateEmbed) -> &'a mut CreateEmbed {
		// TODO see if this constant is defined somewhere
		if self.text.len() <= 4096 {
			create.color(color).description(&self.text)
		} else {
			create
				.color(ERR_COLOR)
				.description("The response was too long and cannot be displayed")
		}
	}

	fn embed_err<'a>(&self, create: &'a mut CreateEmbed) -> &'a mut CreateEmbed {
		self.embed_color(ERR_COLOR, create)
	}

	fn embed_ok<'a>(&self, create: &'a mut CreateEmbed) -> &'a mut CreateEmbed {
		self.embed_color(OK_COLOR, create)
	}

	fn embed<'a>(
		result: Result<&Response, &Response>,
		create: &'a mut CreateEmbed,
	) -> &'a mut CreateEmbed {
		match result {
			Ok(response) => response.embed_ok(create),
			Err(response) => response.embed_err(create),
		}
	}
}

#[async_trait]
pub trait Respond {
	type Value;
	type Error;

	async fn respond(
		&self,
		result: Result<&Response, &Response>,
	) -> Result<Self::Value, Self::Error>;

	async fn respond_ok(&self, response: &Response) -> Result<Self::Value, Self::Error> {
		self.respond(Ok(response)).await
	}

	async fn respond_err(&self, response: &Response) -> Result<Self::Value, Self::Error> {
		self.respond(Err(response)).await
	}
}

#[async_trait]
impl Respond for (&serenity::client::Context, serenity::model::id::ChannelId) {
	type Value = serenity::model::channel::Message;
	type Error = serenity::prelude::SerenityError;

	async fn respond(
		&self,
		result: Result<&Response, &Response>,
	) -> Result<Self::Value, Self::Error> {
		self.1
			.send_message(self.0, |message| {
				message.embed(|embed| Response::embed(result, embed))
			})
			.await
	}
}

#[async_trait]
impl<'a, U, E> Respond for Context<'a, U, E>
where
	U: Sync,
{
	type Value = ReplyHandle<'a>;
	type Error = serenity::prelude::SerenityError;

	async fn respond(
		&self,
		result: Result<&Response, &Response>,
	) -> Result<Self::Value, Self::Error> {
		self.send(|reply| reply.embed(|embed| Response::embed(result, embed)))
			.await
	}
}
