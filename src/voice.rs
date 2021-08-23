use serenity::client::Context;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;
use serenity::model::prelude::{UserId, GuildId};
use serenity::model::interactions::application_command::ApplicationCommandInteraction;

use songbird::error::TrackError;
use songbird::input::Input;
use songbird::tracks::create_player;
use songbird::SongbirdKey;

use std::fmt;
use std::fs::read_dir;
use std::path::{Path, PathBuf};

use itertools::Itertools;

use crate::data::VoiceGuilds;
use crate::util::*;

#[group("voice")]
#[description("Commands to move the bot to voice channels and play clips.")]
#[commands(summon, banish, playnow, queue, volume, stop, skip, playlist)]
pub struct Voice;

pub fn clip_path() -> PathBuf {
	return Path::new("./resources/clips").canonicalize().unwrap();
}

#[derive(Debug)]
pub enum AudioError {
	Songbird(songbird::input::error::Error),
	Serenity(serenity::Error),
}

impl From<songbird::input::error::Error> for AudioError {
	fn from(e: songbird::input::error::Error) -> Self {
		AudioError::Songbird(e)
	}
}

impl From<serenity::Error> for AudioError {
	fn from(e: serenity::Error) -> Self {
		AudioError::Serenity(e)
	}
}

impl fmt::Display for AudioError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Debug::fmt(self, f)
	}
}

impl std::error::Error for AudioError {}

pub async fn audio_source(loc: &str) -> Result<Input, AudioError> {
	Ok(if loc.starts_with("http") {
		songbird::ytdl(&loc).await?
	} else {
		match get_clip(&loc) {
			Some(clip) => songbird::ffmpeg(&clip).await?,
			None => Err(serenity::Error::Other("Could not find source"))?,
		}
	})
}

pub fn get_clip(loc: &str) -> Option<PathBuf> {
	let clip_path = clip_path();
	let mut play_path = clip_path.join(&loc);

	for ext in &["mp3", "wav"] {
		play_path.set_extension(ext);

		if valid_clip(&play_path) {
			return Some(play_path);
		}
	}

	None
}

fn valid_clip(path: &Path) -> bool {
	sandboxed_exists(&clip_path(), &path)
}

async fn summon_generic(ctx: &Context, guild_id: Option<GuildId>, user_id: UserId) -> String {
	let guild_id = match guild_id {
		Some(guild_id) => guild_id,
		None => return "This command is only available in guilds".to_string(),
	};

	let guild = match guild_id.to_guild_cached(&ctx.cache).await {
		Some(guild) => guild,
		None => return "Internal bot error".to_string(),
	};

	let channel_id = guild
		.voice_states
		.get(&user_id)
		.and_then(|voice_state| voice_state.channel_id);

	let connect_to = match channel_id {
		Some(channel_id) => channel_id,
		None => return "Not in a voice channel".to_string(),
	};

	let songbird = ctx
		.data
		.read()
		.await
		.clone_expect::<SongbirdKey>();

	match songbird.join(guild_id, connect_to).await.1 {
		Ok(()) => "Joined channel",
		Err(_) => "Error joining the channel",
	}.to_string()
}

pub async fn summon_interaction(
	ctx: &Context,
	interaction: &ApplicationCommandInteraction,
) -> serenity::Result<()> {
	interaction.respond_str(&ctx, summon_generic(ctx, interaction.guild_id, interaction.user.id).await).await
}

#[command]
#[only_in(guilds)]
#[help_available]
#[description("Summon the bot to the voice channel the user is currently in")]
pub async fn summon(ctx: &Context, msg: &Message) -> CommandResult {
	msg.channel_id.say(&ctx.http, summon_generic(ctx, msg.guild_id, msg.author.id).await).await?;

	Ok(())
}

async fn banish_generic(ctx: &Context, guild_id: Option<GuildId>) -> String {
	let guild_id = match guild_id {
		Some(guild_id) => guild_id,
		None => return "This command is only available in guilds".to_string(),
	};

	let songbird = ctx
		.data
		.read()
		.await
		.clone_expect::<SongbirdKey>();

	{
		use songbird::error::JoinError::*;
		match songbird.remove(guild_id).await {
			Ok(()) => "Left voice channel",
			Err(e) => match e {
				NoCall => "Not in a voice channel",
				_ => "Internal bot error",
			}
		}.to_string()
	}
}

pub async fn banish_interaction(
	ctx: &Context,
	interaction: &ApplicationCommandInteraction,
) -> serenity::Result<()> {
	interaction.respond_str(&ctx, banish_generic(ctx, interaction.guild_id).await).await
}

#[command]
#[only_in(guilds)]
#[help_available]
#[description("Remove the bot from the voice channel it is in")]
pub async fn banish(ctx: &Context, msg: &Message) -> CommandResult {
	msg.channel_id.say(&ctx.http, banish_generic(ctx, msg.guild_id).await).await?;

	Ok(())
}

#[command]
#[aliases(pn)]
#[only_in(guilds)]
#[help_available]
#[description("Play the specified clip immediately")]
#[num_args(1)]
#[usage("<clip>")]
#[example("bnw/needoffspring")]
pub async fn playnow(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	play_generic(ctx, msg, args, PlayType::PlayNow).await
}

#[command]
#[aliases(q)]
#[only_in(guilds)]
#[help_available]
#[description("Add the specified clip to the play queue")]
#[num_args(1)]
#[usage("<clip>")]
#[example("bnw/needoffspring")]
pub async fn queue(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	play_generic(ctx, msg, args, PlayType::Queue).await
}

enum PlayType {
	PlayNow,
	Queue,
}

async fn play_generic(
	ctx: &Context,
	msg: &Message,
	mut args: Args,
	play_type: PlayType,
) -> CommandResult {
	let loc = args.single::<String>().or_err_say(ctx, msg, "Must provide a source").await?;

	let guild_id = msg_guild_id_or_say(ctx, msg).await?;

	{
		let data_lock = ctx.data.read().await;

		let songbird = data_lock
			.clone_expect::<SongbirdKey>();

		let voice_guild_arc = data_lock
			.clone_expect::<VoiceGuilds>()
			.write()
			.await
			.entry(guild_id)
			.or_default()
			.clone();

		let mut voice_guild = voice_guild_arc.write().await;

		if let Some(call) = songbird.get(guild_id) {
			let source = audio_source(&loc).await.or_err_say(ctx, msg, "Invalid clip").await?;

			let (mut track, handle) = create_player(source);
			track.set_volume(voice_guild.volume());

			match play_type {
				PlayType::PlayNow => {
					call.lock().await.play(track);
					voice_guild.add_audio(handle)?;
				}
				PlayType::Queue => {
					call.lock().await.enqueue(track);
				}
			}
		} else {
			msg.channel_id
				.say(&ctx.http, "Not in a voice channel")
				.await?;
		}
	}

	Ok(())
}

#[command]
#[only_in(guilds)]
#[help_available]
#[description("Change volume of bot")]
#[num_args(1)]
#[usage("<volume>")]
#[example("0.5")]
pub async fn volume(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
	let volume = args.single::<f32>().or_err_say(ctx, msg, "Volume must be a valid float between 0.0 and 1.0").await?;

	if volume < 0.0 || volume > 1.0 {
		msg.channel_id
			.say(&ctx.http, "Volume must be between 0.0 and 1.0")
			.await?;
		return Ok(());
	}

	let guild_id = msg_guild_id_or_say(ctx, msg).await?;

	let data_lock = ctx.data.read().await;

	let songbird = data_lock.clone_expect::<SongbirdKey>();

	for handle in songbird.get_or_insert(guild_id.into()).lock().await.queue().current_queue() {
		handle.set_volume(volume).err().filter(|e| e == &TrackError::Finished)
			.and_err_say(
				ctx,
				msg,
				"Error setting volume"
			).await?;
	}


	data_lock
		.clone_expect::<VoiceGuilds>()
		.write()
		.await
		.entry(guild_id)
		.or_default()
		.clone()
		.write()
		.await
		.set_volume(volume)
		.and_say(ctx, msg, format!("Volume set to {}", volume))
		.await?
		.or_say(ctx, msg, "Error setting volume")
		.await??;

	Ok(())
}

#[command]
#[only_in(guilds)]
#[help_available]
#[description("Stop all clips currently being played by the bot")]
pub async fn stop(ctx: &Context, msg: &Message) -> CommandResult {
	let guild_id = msg_guild_id_or_say(ctx, msg).await?;

	ctx
		.data
		.write()
		.await
		.clone_expect::<SongbirdKey>()
		.get_or_insert(guild_id.into())
		.lock()
		.await
		.stop();

	Ok(())
}

#[command]
#[only_in(guilds)]
#[help_available]
#[description("Skip the current song in the queue")]
pub async fn skip(ctx: &Context, msg: &Message) -> CommandResult {
	let guild_id = msg_guild_id_or_say(ctx, msg).await?;

	ctx
		.data
		.write()
		.await
		.clone_expect::<SongbirdKey>()
		.get_or_insert(guild_id.into())
		.lock()
		.await
		.queue()
		.skip()?;

	Ok(())
}

#[command]
#[help_available]
#[description("List all the sections and/or clips available in the section")]
#[min_args(0)]
#[max_args(1)]
#[usage("[section]")]
#[example("bnw")]
pub async fn playlist(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	if args.len() > 1 {
		msg.channel_id
			.say(&ctx.http, "Expected at most one path to be specified")
			.await?;
		return Ok(());
	}

	let dir = clip_path().join(Path::new(match args.len() {
		0 => "",
		1 => args.current().unwrap(),
		_ => {
			eprintln!("Unexpected number of arguments");
			return Ok(());
		}
	}));

	let dir = match dir.canonicalize() {
		Ok(dir) => dir,
		Err(_reason) => {
			msg.channel_id.say(&ctx.http, "Invalid directory").await?;
			return Ok(());
		}
	};

	if !sandboxed_exists(&clip_path(), &dir) {
		msg.channel_id.say(&ctx.http, "Invalid directory").await?;
		return Ok(());
	}

	match read_dir(dir) {
		Err(reason) => {
			eprintln!("Unable to read directory: {:?}", reason);
			msg.channel_id.say(&ctx.http, "Invalid directory").await?;
			return Ok(());
		}
		Ok(dir_iter) => {
			let message = dir_iter
				.filter_map(|e| e.ok())
				.map(|e| {
					(
						e.path()
							.file_stem()
							.and_then(|f| f.to_str())
							.map(|f| f.to_owned()),
						e.file_type(),
					)
				})
				.filter(|(f, t)| f.is_some() && t.is_ok())
				.map(|(f, t)| (f.unwrap(), t.unwrap()))
				.sorted_by(|(f0, t0), (f1, t1)| {
					(!t0.is_dir(), f0.to_lowercase()).cmp(&(!t1.is_dir(), f1.to_lowercase()))
				})
				.map(|(f, t)| format!("{: <20}", f + if t.is_dir() { "/" } else { "" }))
				.chunks(3)
				.into_iter()
				.map(|chunk| chunk.fold("".to_owned(), |acc, s| acc + &s))
				.fold("".to_owned(), |acc, s| acc + "\n" + &s);

			msg.channel_id
				.say(&ctx.http, "```\n".to_owned() + &message + "\n```")
				.await?;
		}
	}

	return Ok(());
}
