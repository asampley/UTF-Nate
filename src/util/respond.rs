use serenity::async_trait;
use serenity::builder::CreateEmbed;
use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::model::interactions::application_command::ApplicationCommandInteraction;
use serenity::utils::Color;

#[derive(Debug)]
pub struct Response {
	pub text: String,
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
		create.color(color).description(&self.text)
	}

	fn embed_err<'a>(&self, create: &'a mut CreateEmbed) -> &'a mut CreateEmbed {
		self.embed_color(Color::from_rgb(255, 127, 127), create)
	}

	fn embed_ok<'a>(&self, create: &'a mut CreateEmbed) -> &'a mut CreateEmbed {
		self.embed_color(Color::from_rgb(127, 127, 255), create)
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

	async fn respond(
		&self,
		ctx: &Context,
		result: Result<&Response, &Response>,
	) -> serenity::Result<Self::Value>;

	async fn respond_ok(
		&self,
		ctx: &Context,
		response: &Response,
	) -> serenity::Result<Self::Value> {
		self.respond(ctx, Ok(response)).await
	}

	async fn respond_err(
		&self,
		ctx: &Context,
		response: &Response,
	) -> serenity::Result<Self::Value> {
		self.respond(ctx, Err(response)).await
	}
}

#[async_trait]
impl Respond for Message {
	type Value = Message;

	async fn respond(
		&self,
		ctx: &Context,
		result: Result<&Response, &Response>,
	) -> serenity::Result<Self::Value> {
		self.channel_id
			.send_message(&ctx.http, |message| {
				message.embed(|embed| Response::embed(result, embed))
			})
			.await
	}
}

#[async_trait]
impl Respond for ApplicationCommandInteraction {
	type Value = ();

	async fn respond(
		&self,
		ctx: &Context,
		result: Result<&Response, &Response>,
	) -> serenity::Result<Self::Value> {
		self.create_interaction_response(&ctx.http, |response| {
			response.interaction_response_data(|data| {
				data.create_embed(|embed| Response::embed(result, embed))
			})
		})
		.await
	}
}
