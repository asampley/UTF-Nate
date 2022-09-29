use std::collections::HashSet;

use itertools::Itertools;

use rand::Rng;

use serenity::client::Context;
use serenity::model::prelude::GuildId;

use songbird::SongbirdKey;

use thiserror::Error;

use tracing::{error, info};

use crate::data::VoiceGuilds;
use crate::parser::{NumOrRange, Selection};
use crate::util::{GetExpect, Response};

#[tracing::instrument(level = "info", ret, skip(ctx))]
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
		.get_or_insert(guild_id)
		.lock()
		.await
		.queue()
		.stop();

	Ok("Cleared queue and stopped playing".into())
}

#[tracing::instrument(level = "info", ret, skip(ctx))]
pub async fn skip(
	ctx: &Context,
	guild_id: Option<GuildId>,
	skip_set: Option<Selection<usize>>,
) -> Result<Response, Response> {
	let guild_id = guild_id.ok_or("This command is only available in guilds")?;

	let call = ctx
		.data
		.read()
		.await
		.clone_expect::<SongbirdKey>()
		.get_or_insert(guild_id)
		.clone();

	let call = call.lock().await;

	let queue = call.queue();

	let result = match skip_set {
		Some(mut skip_set) if !skip_set.0.is_empty() => {
			let mut removed = HashSet::new();

			queue.modify_queue(|deque| {
				skip_set.0.sort_unstable_by_key(|s| match s {
					NumOrRange::Num(n) => *n,
					NumOrRange::Range(r) => *r.end(),
				});

				// remove in reverse order to prevent indices from shifting
				for s in skip_set.0.into_iter().rev() {
					if deque.is_empty() {
						continue;
					}

					match s {
						NumOrRange::Num(n) => {
							if !removed.contains(&n) && n < deque.len() {
								deque.remove(n).map(|q| q.stop());
								removed.insert(n);
							}
						}
						NumOrRange::Range(r) => {
							// remove in reverse order to prevent indices from shifting
							for i in r.into_iter().rev() {
								if !removed.contains(&i) && i < deque.len() {
									deque.remove(i).map(|q| q.stop());
									removed.insert(i);
								}
							}
						}
					}
				}
			});

			info!("Skipped tracks {:?}", &removed);

			queue.resume().map(|_| removed.len())
		}
		_ => {
			if queue.is_empty() {
				info!("No tracks to skip");
				Ok(0)
			} else {
				info!("Skipped first track");
				queue.skip().map(|_| 1)
			}
		}
	};

	result
		.map(|count| match count {
			0 => "No clips skipped".into(),
			1 => "Skipped 1 clip".into(),
			c => format!("Skipped {} clips", c).into(),
		})
		.map_err(|e| {
			error!("{:?}", e);
			"Error skipping clips".into()
		})
}

#[tracing::instrument(level = "info", ret, skip(ctx))]
pub async fn pause(ctx: &Context, guild_id: Option<GuildId>) -> Result<Response, Response> {
	let guild_id = guild_id.ok_or("This command is only available in guilds")?;

	ctx.data
		.read()
		.await
		.clone_expect::<SongbirdKey>()
		.get_or_insert(guild_id)
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

#[tracing::instrument(level = "info", ret, skip(ctx))]
pub async fn unpause(ctx: &Context, guild_id: Option<GuildId>) -> Result<Response, Response> {
	let guild_id = guild_id.ok_or("This command is only available in guilds")?;

	ctx.data
		.read()
		.await
		.clone_expect::<SongbirdKey>()
		.get_or_insert(guild_id)
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

#[tracing::instrument(level = "info", ret, skip(ctx))]
pub async fn queue(ctx: &Context, guild_id: Option<GuildId>) -> Result<Response, Response> {
	let guild_id = guild_id.ok_or("This command is only available in guilds")?;

	let call = ctx
		.data
		.read()
		.await
		.clone_expect::<SongbirdKey>()
		.get_or_insert(guild_id);

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

#[tracing::instrument(level = "info", ret, skip(ctx))]
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
		.get_or_insert(guild_id)
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
			"Error shuffling queue".into()
		})
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoopArg {
	On,
	Off,
	Count(usize),
}

#[derive(Debug, Error)]
#[error("expected \"on\", \"off\", or an integer")]
pub struct ParseLoopArgError;

impl core::str::FromStr for LoopArg {
	type Err = ParseLoopArgError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(if s.eq_ignore_ascii_case("on") {
			LoopArg::On
		} else if s.eq_ignore_ascii_case("off") {
			LoopArg::Off
		} else {
			LoopArg::Count(s.parse().map_err(|_| ParseLoopArgError)?)
		})
	}
}

#[tracing::instrument(level = "info", ret, skip(ctx))]
pub async fn r#loop(
	ctx: &Context,
	guild_id: Option<GuildId>,
	loop_arg: LoopArg,
) -> Result<Response, Response> {
	let guild_id = guild_id.ok_or("This command is only available in guilds")?;

	let call = ctx
		.data
		.read()
		.await
		.clone_expect::<SongbirdKey>()
		.get_or_insert(guild_id)
		.clone();

	let call = call.lock().await;

	let queue = call.queue();

	let current = queue.current().ok_or("Nothing is currently playing")?;

	match loop_arg {
		LoopArg::On => current.enable_loop().map(|_| "Looping current song".into()),
		LoopArg::Off => current
			.disable_loop()
			.map(|_| "No longer looping current song".into()),
		LoopArg::Count(c) => current
			.loop_for(c)
			.map(|_| format!("Looping current song {c} more times").into()),
	}
	.map_err(|e| {
		error!("{:?}", e);
		"Error changing looping settings".into()
	})
}
