use std::cmp::min;
use std::collections::HashSet;

use rand::Rng;

use serde::{Deserialize, Serialize};

use songbird::tracks::PlayMode;
use songbird::SongbirdKey;

use tap::TapFallible;
use thiserror::Error;

use tracing::{error, info};

use crate::commands::{BotState, Source};
use crate::data::{TrackMetadata, VoiceGuilds};
use crate::parser::{NumOrRange, Selection};
use crate::util::{write_track, GetExpect, Response};

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

pub const fn move_help() -> &'static str {
	include_str!("move.md")
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

#[derive(Debug, Deserialize, Serialize)]
pub struct MoveArgs {
	pub selection: Selection<usize>,
	pub position: usize,
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

	let current = queue.current().ok_or("Nothing is currently playing")?;

	let resume = current
		.get_info()
		.await
		.tap_err(|e| error!("{:?}", e))
		.map_err(|_| "Error skipping clips")?
		.playing == PlayMode::Play;

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

			if resume {
				queue.resume().map(|_| removed.len())
			} else {
				Ok(removed.len())
			}
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
		.tap_err(|e| error!("{:?}", e))
		.map_err(|_| "Error skipping clips".into())
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
		.tap_err(|e| error!("{:?}", e))
		.map_err(|_| "Error pausing clip".into())
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
		.tap_err(|e| error!("{:?}", e))
		.map_err(|_| "Error unpausing clip".into())
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

	let lock = state.data.read().await;

	let call = lock.clone_expect::<SongbirdKey>().get_or_insert(guild_id);
	let call_lock = call.lock().await;

	let current_queue = call_lock.queue().current_queue();
	let len = current_queue.len();

	if len == 0 {
		return Ok("Nothing queued".into());
	}

	let mut response = format!("Current queue ({} total):\n", len);

	let tracks = args
		.selection
		.0
		.iter()
		.flat_map(|v| match v {
			NumOrRange::Num(n) => *n..=*n,
			NumOrRange::Range(r) => r.clone(),
		})
		.filter_map(|i| current_queue.get(i).map(|t| (i, t)));

	for (i, track) in tracks {
		use std::fmt::Write;

		write!(response, "{i}:").unwrap();

		if let Some(meta) = track.typemap().read().await.get::<TrackMetadata>() {
			write_track(&mut response, meta, track.get_info().await.ok()).unwrap();
		} else {
			response.push_str("No metadata");
		}

		writeln!(response).unwrap();
	}

	Ok(response.into())
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
		.tap_err(|e| error!("{:?}", e))
		.map_err(|_| "Error shuffling queue".into())
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
	.tap_err(|e| error!("{:?}", e))
	.map_err(|_| "Error changing looping settings".into())
}

pub async fn r#move(
	state: &BotState,
	source: &Source,
	args: MoveArgs,
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

	let resume = current
		.get_info()
		.await
		.tap_err(|e| error!("{:?}", e))
		.map_err(|_| "Error skipping clips")?
		.playing == PlayMode::Play;

	if args.position == 0 {
		queue
			.pause()
			.tap_err(|e| error!("{:?}", e))
			.map_err(|_| "Error moving clips")?;
	}

	let moved = queue.modify_queue(|deque| {
		let selection_iter = args.selection.into_iter();

		// once accounted for moving, don't move twice
		let moving: HashSet<_> = selection_iter.clone().collect();

		let mut indices = vec![usize::MAX; deque.len()];

		// position can at most be the length of the queue less the size of the selection
		let position = min(deque.len() - moving.len(), args.position);

		// fill in selection indices first
		let mut dest = position;
		for s in selection_iter {
			if s < deque.len() && indices[s] == usize::MAX {
				indices[s] = dest;
				dest += 1;
			}
		}

		// fill in the rest of the indices
		let mut dest_rest = 0;
		for i in &mut indices {
			// skip to end of selection if we've hit the start
			if dest_rest == position {
				dest_rest = dest;
			}
			// change anything not yet set
			if *i == usize::MAX {
				*i = dest_rest;
				dest_rest += 1;
			}
		}

		// swap element until everything is in order
		// this will terminate because each step puts one
		// more element in the correct place, and it finishes
		// when all elements are in the correct place.
		let mut i = 0;
		while i < indices.len() {
			let goto = indices[i];

			if i == goto {
				i += 1;
			} else {
				deque.swap(i, goto);
				indices.swap(i, goto);
			}
		}

		moving.len()
	});

	info!("Moved tracks {:?}", moved);

	let result = if resume { queue.resume() } else { Ok(()) };

	result
		.map(|_| match moved {
			0 => "No clips moved".into(),
			1 => "Moved 1 clip".into(),
			c => format!("Moved {} clips", c).into(),
		})
		.tap_err(|e| error!("{:?}", e))
		.map_err(|_| "Error moving clips".into())
}
