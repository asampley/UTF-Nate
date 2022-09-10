use serenity::client::Context;
use serenity::model::prelude::{GuildId, UserId};

use songbird::SongbirdKey;

use crate::util::*;

#[tracing::instrument(level = "info", ret, skip(ctx))]
pub async fn summon(
	ctx: &Context,
	guild_id: Option<GuildId>,
	user_id: UserId,
) -> Result<Response, Response> {
	let guild_id = guild_id.ok_or("This command is only available in guilds")?;
	let guild = guild_id
		.to_guild_cached(&ctx.cache)
		.ok_or("Internal bot error")?;

	let channel_id = guild
		.voice_states
		.get(&user_id)
		.and_then(|voice_state| voice_state.channel_id);

	let connect_to = channel_id.ok_or("Not in a voice channel")?;

	let songbird = ctx.data.read().await.clone_expect::<SongbirdKey>();
	let (_call, join_result) = songbird.join(guild_id, connect_to).await;

	match join_result {
		Ok(()) => Ok("Joined channel".into()),
		Err(_) => Err("Error joining the channel".into()),
	}
}

#[tracing::instrument(level = "info", ret, skip(ctx))]
pub async fn banish(ctx: &Context, guild_id: Option<GuildId>) -> Result<Response, Response> {
	let guild_id = guild_id.ok_or("This command is only available in guilds")?;

	let songbird = ctx.data.read().await.clone_expect::<SongbirdKey>();

	{
		use songbird::error::JoinError::*;
		match songbird.remove(guild_id).await {
			Ok(()) => Ok("Left voice channel".into()),
			Err(e) => match e {
				NoCall => Err("Not in a voice channel".into()),
				_ => Err("Internal bot error".into()),
			},
		}
	}
}
