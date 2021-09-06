use serenity::async_trait;
use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::model::interactions::application_command::ApplicationCommandInteraction;

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
