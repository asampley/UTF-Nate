use rand::{thread_rng, Rng};

use serenity::client::Context;
use serenity::model::prelude::{GuildId, UserId};

use songbird::SongbirdKey;

use crate::data::VoiceGuilds;
use crate::util::{GetExpect, Response};

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

	let data_lock = ctx.data.read().await;

	let voice_guild = data_lock
		.clone_expect::<VoiceGuilds>()
		.entry(guild_id)
		.or_default()
		.clone();

	let songbird = data_lock.clone_expect::<SongbirdKey>();

	drop(data_lock);

	if songbird.get(guild_id).is_none() {
		return Err("Not in a voice channel".into());
	}

	let banish_attempts = &mut voice_guild.write().await.banish_attempts;
	*banish_attempts = (*banish_attempts + 1) % 3;

	match banish_attempts {
		1 => Err("5 more minutes \u{1F97A}".into()),
		2 => Err(
			"Please, don't send me back there, it's dark, and scary, and so very very quiet."
				.into(),
		),
		_ => {
			let mut message = "NOOOOOOOOOOOOO".to_string();

			{
				let mut drng = thread_rng();
				let mut rng = thread_rng();

				let mut diacritics = (0..)
					.map(move |_| drng.gen_range(0x0300..=0x036F))
					.filter(|c| *c != 0x034F)
					.map(char::from_u32);

				let mut i = 0;
				while i < message.len() {
					if rng.gen_range(0..=1) == 1 {
						i += 1;
						message.insert(i, diacritics.next().unwrap().unwrap());
					}

					i += 1;
				}
			}

			use songbird::error::JoinError::*;
			match songbird.remove(guild_id).await {
				Ok(()) => Ok(message.into()),
				Err(e) => match e {
					NoCall => Err("Not in a voice channel".into()),
					_ => Err("Internal bot error".into()),
				},
			}
		}
	}
}
