use songbird::SongbirdKey;

use tracing::error;

use crate::commands::{BotState, Source};
use crate::util::{GetExpect, Response};

#[cfg(feature = "http-interface")]
pub mod http;
pub mod poise;

pub const fn summon_help() -> &'static str {
	include_str!("help/summon.md")
}

pub const fn banish_help() -> &'static str {
	include_str!("help/banish.md")
}

#[tracing::instrument(level = "info", ret, skip(state))]
pub async fn summon(state: &BotState, source: &Source) -> Result<Response, Response> {
	let guild_id: serenity::model::id::GuildId = source
		.guild_id
		.ok_or("This command is only available in guilds")?;

	let channel_id = guild_id
		.to_guild_cached(&state.cache)
		.ok_or("Internal bot error")?
		.voice_states
		.get(&source.user_id)
		.and_then(|voice_state| voice_state.channel_id);

	let connect_to = channel_id.ok_or("Not in a voice channel")?;

	let songbird = state.data.read().await.clone_expect::<SongbirdKey>();
	songbird
		.join(guild_id, connect_to)
		.await
		.inspect_err(|e| error!("Error joining the channel: {e:?}"))
		.map_err(|_| "Error joining the channel")?;

	Ok("Joined channel".into())
}

#[tracing::instrument(level = "info", ret, skip(state))]
pub async fn banish(state: &BotState, source: &Source) -> Result<Response, Response> {
	let guild_id = source
		.guild_id
		.ok_or("This command is only available in guilds")?;

	let songbird = state.data.read().await.clone_expect::<SongbirdKey>();

	{
		use songbird::error::JoinError::*;
		match songbird.remove(guild_id).await {
			Ok(()) => Ok("Left voice channel".into()),
			Err(e) => match e {
				NoCall => Err("Not in a voice channel".into()),
				e => {
					error!("Error while joining the call: {:?}", e);
					Err("Internal bot error".into())
				}
			},
		}
	}
}
