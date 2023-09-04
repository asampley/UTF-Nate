use std::collections::HashSet;

use itertools::Itertools;

use rand::Rng;

use serde::{Deserialize, Serialize};

use songbird::SongbirdKey;

use thiserror::Error;

use tracing::{error, info};

use crate::commands::{BotState, Source};
use crate::data::VoiceGuilds;
use crate::parser::{NumOrRange, Selection};
use crate::util::{GetExpect, Response};

#[cfg(feature = "http-interface")]
pub mod http;
pub mod poise;

pub const fn stop_help() -> &'static str {
	include_str!("stop.md")
}

pub const fn skip_help() -> &'static str {
	include_str!("skip.md")
}

pub const fn pause_help() -> &'static str {
	include_str!("pause.md")
}

pub const fn unpause_help() -> &'static str {
	include_str!("unpause.md")
}

pub const fn queue_help() -> &'static str {
	include_str!("queue.md")
}

pub const fn shuffle_help() -> &'static str {
	include_str!("shuffle.md")
}

pub const fn shufflenow_help() -> &'static str {
	include_str!("shufflenow.md")
}

pub const fn loop_help() -> &'static str {
	include_str!("loop.md")
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SkipArgs {
	pub skip_set: Option<Selection<usize>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoopArgs {
	pub count: LoopArg,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QueueArgs {
	#[serde(default = "QueueArgs::default_selection")]
	pub selection: Selection<usize>,
}

impl QueueArgs {
	fn default_selection() -> Selection<usize> {
		Selection(vec![NumOrRange::Range(0..=10)])
	}
}

impl Default for QueueArgs {
	fn default() -> Self {
		Self {
			selection: Self::default_selection(),
		}
	}
}

#[tracing::instrument(level = "info", ret, skip(state))]
pub async fn stop(state: &BotState, source: &Source) -> Result<Response, Response> {
	let guild_id = source
		.guild_id
		.ok_or("This command is only available in guilds")?;

	let lock = state.data.read().await;

	if let Some(voice_guild) = lock.clone_expect::<VoiceGuilds>().get(&guild_id) {
		voice_guild.write().await.stop()
	}

	state
		.data
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

#[tracing::instrument(level = "info", ret, skip(state))]
pub async fn skip(
	state: &BotState,
	source: &Source,
	args: &SkipArgs,
) -> Result<Response, Response> {
	let guild_id = source
		.guild_id
		.ok_or("This command is only available in guilds")?;

	let call = state
		.data
		.read()
		.await
		.clone_expect::<SongbirdKey>()
		.get_or_insert(guild_id)
		.clone();

	let call = call.lock().await;

	let queue = call.queue();

	let result = match &args.skip_set {
		Some(skip_set) if !skip_set.0.is_empty() => {
			let mut skip_set = skip_set.clone();
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

#[tracing::instrument(level = "info", ret, skip(state))]
pub async fn pause(state: &BotState, source: &Source) -> Result<Response, Response> {
	let guild_id = source
		.guild_id
		.ok_or("This command is only available in guilds")?;

	state
		.data
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

#[tracing::instrument(level = "info", ret, skip(state))]
pub async fn unpause(state: &BotState, source: &Source) -> Result<Response, Response> {
	let guild_id = source
		.guild_id
		.ok_or("This command is only available in guilds")?;

	state
		.data
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

#[tracing::instrument(level = "info", ret, skip(state))]
pub async fn queue(
	state: &BotState,
	source: &Source,
	args: QueueArgs,
) -> Result<Response, Response> {
	let guild_id = source
		.guild_id
		.ok_or("This command is only available in guilds")?;

	let call = state
		.data
		.read()
		.await
		.clone_expect::<SongbirdKey>()
		.get_or_insert(guild_id);

	let lock = call.lock().await;

	let current_queue = lock.queue().current_queue();
	let len = current_queue.len();

	Ok(if len == 0 {
		"Nothing queued".to_owned()
	} else {
		let queue = args
			.selection
			.0
			.iter()
			.flat_map(|v| match v {
				NumOrRange::Num(n) => *n..=*n,
				NumOrRange::Range(r) => r.clone(),
			})
			.filter_map(|i| current_queue.get(i).map(|t| (i, t)))
			.map(|(i, track)| {
				let meta = track.metadata();
				let title = meta.title.as_deref().unwrap_or("Unknown");
				match &meta.source_url {
					Some(url) => format!("{}: [{}]({})", i, title, url),
					None => format!("{}: {}", i, title),
				}
			})
			.join("\n");

		format!("Current queue ({} total):\n{}\n", len, queue)
	}
	.into())
}

#[tracing::instrument(level = "info", ret, skip(state))]
pub async fn shuffle(
	state: &BotState,
	source: &Source,
	shuffle_from: usize,
) -> Result<Response, Response> {
	let guild_id = source
		.guild_id
		.ok_or("This command is only available in guilds")?;

	let call = state
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

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
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

#[tracing::instrument(level = "info", ret, skip(state))]
pub async fn r#loop(
	state: &BotState,
	source: &Source,
	args: &LoopArgs,
) -> Result<Response, Response> {
	let guild_id = source
		.guild_id
		.ok_or("This command is only available in guilds")?;

	let call = state
		.data
		.read()
		.await
		.clone_expect::<SongbirdKey>()
		.get_or_insert(guild_id)
		.clone();

	let call = call.lock().await;

	let queue = call.queue();

	let current = queue.current().ok_or("Nothing is currently playing")?;

	match &args.count {
		LoopArg::On => current.enable_loop().map(|_| "Looping current song".into()),
		LoopArg::Off => current
			.disable_loop()
			.map(|_| "No longer looping current song".into()),
		LoopArg::Count(c) => current
			.loop_for(*c)
			.map(|_| format!("Looping current song {c} more times").into()),
	}
	.map_err(|e| {
		error!("{:?}", e);
		"Error changing looping settings".into()
	})
}
