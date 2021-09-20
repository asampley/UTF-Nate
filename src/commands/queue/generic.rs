use itertools::Itertools;

use log::error;

use rand::Rng;

use serenity::client::Context;
use serenity::model::prelude::GuildId;

use songbird::SongbirdKey;

use crate::data::VoiceGuilds;
use crate::util::*;

pub async fn stop(ctx: &Context, guild_id: Option<GuildId>) -> Result<Response, Response> {
	let guild_id = guild_id.ok_or("This command is only available in guilds")?;

	let lock = ctx.data.read().await;

	if let Some(voice_guild) = lock.clone_expect::<VoiceGuilds>().get(&guild_id) {
		voice_guild.write().await.stop()
	}

	ctx.data
		.read()
		.await
		.clone_expect::<SongbirdKey>()
		.get_or_insert(guild_id.into())
		.lock()
		.await
		.queue()
		.stop();

	Ok("Cleared queue and stopped playing".into())
}

pub async fn skip(
	ctx: &Context,
	guild_id: Option<GuildId>,
	skip_count: Option<usize>,
) -> Result<Response, Response> {
	let guild_id = guild_id.ok_or("This command is only available in guilds")?;

	let call = ctx
		.data
		.read()
		.await
		.clone_expect::<SongbirdKey>()
		.get_or_insert(guild_id.into())
		.clone();

	let call = call.lock().await;

	let queue = call.queue();

	let result = if let Some(count) = skip_count {
		queue
			.modify_queue(|deque| {
				(0..count)
					.filter_map(|_| deque.pop_front())
					.fuse()
					.map(|queued| queued.stop())
					.fold_ok(0, |acc, _| acc + 1)
			})
			.and_then(|count| queue.resume().map(|_| count))
	} else {
		queue.skip().map(|_| 1)
	};

	result
		.map(|count| match count {
			1 => "Skipped current clip".into(),
			_ => format!("Skipped {} clips", count).into(),
		})
		.map_err(|e| {
			error!("{:?}", e);
			"Error skipping clips".into()
		})
}

pub async fn pause(ctx: &Context, guild_id: Option<GuildId>) -> Result<Response, Response> {
	let guild_id = guild_id.ok_or("This command is only available in guilds")?;

	ctx.data
		.read()
		.await
		.clone_expect::<SongbirdKey>()
		.get_or_insert(guild_id.into())
		.lock()
		.await
		.queue()
		.pause()
		.map(|_| "Pausing current clip".into())
		.map_err(|e| {
			error!("{:?}", e);
			"Error pausing clip".into()
		})
}

pub async fn unpause(ctx: &Context, guild_id: Option<GuildId>) -> Result<Response, Response> {
	let guild_id = guild_id.ok_or("This command is only available in guilds")?;

	ctx.data
		.read()
		.await
		.clone_expect::<SongbirdKey>()
		.get_or_insert(guild_id.into())
		.lock()
		.await
		.queue()
		.resume()
		.map(|_| "Unpausing current clip".into())
		.map_err(|e| {
			error!("{:?}", e);
			"Error unpausing clip".into()
		})
}

pub async fn queue(ctx: &Context, guild_id: Option<GuildId>) -> Result<Response, Response> {
	let guild_id = guild_id.ok_or("This command is only available in guilds")?;

	let call = ctx
		.data
		.read()
		.await
		.clone_expect::<SongbirdKey>()
		.get_or_insert(guild_id.into());

	let lock = call.lock().await;

	let current_queue = lock.queue().current_queue();
	let len = current_queue.len();

	Ok(if len == 0 {
		format!("Nothing queued")
	} else {
		let queue = current_queue
			.into_iter()
			.take(10)
			.enumerate()
			.map(|(i, t)| {
				let m = t.metadata();
				let t = m.title.as_deref().unwrap_or("Unknown");
				match &m.source_url {
					Some(u) => format!("{}: [{}]({})", i, t, u),
					None => format!("{}: {}", i, t),
				}
			})
			.join("\n");

		if len <= 10 {
			format!("Current queue:\n{}", queue).into()
		} else {
			format!("Current queue:\n{}\n... and {} more", queue, len - 10)
		}
	}
	.into())
}

pub async fn shuffle(
	ctx: &Context,
	guild_id: Option<GuildId>,
	shuffle_from: usize,
) -> Result<Response, Response> {
	let guild_id = guild_id.ok_or("This command is only available in guilds")?;

	let call = ctx
		.data
		.read()
		.await
		.clone_expect::<SongbirdKey>()
		.get_or_insert(guild_id.into())
		.clone();

	let call = call.lock().await;

	let queue = call.queue();

	queue
		.pause()
		.map(|_| {
			queue.modify_queue(|deque| {
				let mut rng = rand::thread_rng();

				for j in ((shuffle_from + 1)..deque.len()).rev() {
					let i = rng.gen_range(shuffle_from..=j);
					deque.swap(i, j);
				}
			})
		})
		.and_then(|_| queue.resume())
		.map(|_| "Shuffled queue".into())
		.map_err(|e| {
			error!("{:?}", e);
			"Error shuflling queue".into()
		})
}
