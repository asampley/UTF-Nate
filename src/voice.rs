use futures::Future;

use itertools::Itertools;

use log::{debug, error, warn};

use once_cell::sync::Lazy;

use serde_json::value::Value;

use regex::Regex;

use reqwest::Url;

use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;
use serenity::model::interactions::application_command::{
	ApplicationCommandInteraction, ApplicationCommandOptionType,
};

use songbird::input::restartable::Restartable;
use songbird::input::Input;

use walkdir::WalkDir;

use std::cmp;
use std::fmt;
use std::path::{Path, PathBuf};

use crate::data::Keys;
use crate::util::*;
use crate::youtube;

mod generic;

static URL: Lazy<Regex> = Lazy::new(|| Regex::new("^https?://").unwrap());

static YOUTUBE_HOST: Lazy<Regex> =
	Lazy::new(|| Regex::new("^((www|m)\\.youtube\\.com|youtu.be)").unwrap());

static SPOTIFY_HOST: Lazy<Regex> = Lazy::new(|| Regex::new("^open\\.spotify\\.com").unwrap());

#[group("voice")]
#[description("Commands to move the bot to voice channels and play clips.")]
#[commands(summon, banish, clip, play, volume, stop, skip, list)]
pub struct Voice;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PlayStyle {
	Play,
	Clip,
}

impl std::str::FromStr for PlayStyle {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_ascii_lowercase().as_str() {
			"play" => Ok(PlayStyle::Play),
			"clip" => Ok(PlayStyle::Clip),
			_ => Err(()),
		}
	}
}

pub fn clip_path() -> PathBuf {
	return Path::new("./resources/clips").canonicalize().unwrap();
}

#[derive(Debug)]
pub enum AudioError {
	Songbird(songbird::input::error::Error),
	Spotify,
	YoutubePlaylist,
	UnsupportedUrl,
	MultipleClip,
	NoClip,
}

impl From<songbird::input::error::Error> for AudioError {
	fn from(e: songbird::input::error::Error) -> Self {
		AudioError::Songbird(e)
	}
}

impl fmt::Display for AudioError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fmt::Debug::fmt(self, f)
	}
}

impl std::error::Error for AudioError {}

pub async fn clip_source(loc: &str) -> Result<Input, AudioError> {
	match find_clip(&loc) {
		FindClip::One(clip) => match get_clip(&clip) {
			Some(clip) => Ok(songbird::ffmpeg(&clip).await?),
			None => Err(AudioError::NoClip),
		},
		FindClip::Multiple => Err(AudioError::MultipleClip),
		FindClip::None => Err(AudioError::NoClip),
	}
}

pub async fn play_sources<F, T>(keys: &Keys, loc: &str, f: F) -> Result<usize, AudioError>
where
	F: Fn(Input) -> T + Send + Sync + 'static,
	T: Future + Send,
{
	if URL.is_match(loc) {
		let url = Url::parse(loc).map_err(|_| AudioError::UnsupportedUrl)?;
		let host = url.host_str().ok_or(AudioError::UnsupportedUrl)?;

		if YOUTUBE_HOST.is_match(host) {
			let path = url.path();

			// if it is a playlist, queue the playlist
			if path == "/playlist" {
				let id = url
					.query_pairs()
					.filter(|(key, _)| key == "list")
					.map(|(_, value)| value)
					.next()
					.ok_or(AudioError::UnsupportedUrl)?;

				let youtube_api = &keys
					.youtube_api
					.as_ref()
					.ok_or(AudioError::YoutubePlaylist)?;

				let playlist = youtube::playlist(youtube_api, &id).await;

				debug!("Youtube playlist: {:#?}", playlist);

				let playlist = playlist.map_err(|_| AudioError::YoutubePlaylist)?;

				let count = playlist.items.len();

				tokio::spawn(async move {
					for item in playlist.items {
						let result = youtube::YtdlLazy::from_item(&item).as_input().await;

						match result {
							Ok(input) => {
								f(input).await;
							}
							Err(e) => error!("Error creating input: {:?}", e),
						}
					}
				});

				Ok(count)
			} else {
				let loc_string = loc.to_string();
				tokio::spawn(async move {
					let result = Restartable::ytdl(loc_string, true).await;

					match result {
						Ok(restartable) => {
							f(restartable.into()).await;
						}
						Err(e) => error!("Error creating input: {:?}", e),
					}
				});
				Ok(1)
			}
		} else if SPOTIFY_HOST.is_match(host) {
			Err(AudioError::Spotify)
		} else {
			Err(AudioError::UnsupportedUrl)
		}
	} else {
		let loc_string = loc.to_string();
		tokio::spawn(async move {
			let result = Restartable::ytdl_search(loc_string, true).await;

			match result {
				Ok(restartable) => {
					f(restartable.into()).await;
				}
				Err(e) => error!("Error creating input: {:?}", e),
			}
		});
		Ok(1)
	}
}

pub enum FindClip {
	One(String),
	Multiple,
	None,
}

#[derive(Debug)]
struct OrdKey<K, V> {
	key: K,
	value: V,
}

impl<K, V> PartialEq for OrdKey<K, V>
where
	K: PartialEq,
{
	fn eq(&self, other: &Self) -> bool {
		self.key.eq(&other.key)
	}
}

impl<K, V> Eq for OrdKey<K, V> where K: Eq {}

impl<K, V> PartialOrd for OrdKey<K, V>
where
	K: PartialOrd,
{
	fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
		self.key.partial_cmp(&other.key)
	}
}

impl<K, V> Ord for OrdKey<K, V>
where
	K: Ord,
{
	fn cmp(&self, other: &Self) -> cmp::Ordering {
		self.key.cmp(&other.key)
	}
}

pub fn warn_duplicate_clip_names() {
	let clip_path = clip_path();

	WalkDir::new(&clip_path)
		.into_iter()
		.filter_map(|f| f.ok())
		.filter(|f| f.file_type().is_file())
		.map(|f| f.path().file_stem().unwrap().to_string_lossy().into_owned())
		.duplicates()
		.for_each(|s| warn!("Multiple clips have the name \"{}\"", s));
}

pub fn find_clip(loc: &str) -> FindClip {
	let clip_path = clip_path();
	let components = loc.split('/').collect_vec();

	let top_two = WalkDir::new(&clip_path)
		.into_iter()
		.filter_map(|f| f.ok())
		.filter(|f| f.file_type().is_file())
		// The name of the clip must match exactly in the path
		.filter(|f| {
			components
				.iter()
				.contains(&&*f.path().file_stem().unwrap().to_string_lossy())
		})
		// count the number of components in a path which match the supplied components
		// the highest score becomes the clip
		// a tie results in no clip returned
		.map(|f| OrdKey {
			key: -(f
				.path()
				.components()
				.filter(|c| {
					components
						.iter()
						.any(|d| d == &c.as_os_str().to_string_lossy())
				})
				.count() as isize),
			value: f,
		})
		.k_smallest(2)
		.collect_vec();

	debug!("Found the follwing top two clips: {:?}", top_two);

	if top_two.len() == 0 {
		FindClip::None
	} else if top_two.len() > 1 && top_two[0].key == top_two[1].key {
		FindClip::Multiple
	} else {
		FindClip::One(
			top_two[0]
				.value
				.path()
				.strip_prefix(&clip_path)
				.unwrap()
				.with_extension("")
				.to_string_lossy()
				.into_owned(),
		)
	}
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

#[command]
#[only_in(guilds)]
#[help_available]
#[description("Summon the bot to the voice channel the user is currently in")]
pub async fn summon(ctx: &Context, msg: &Message) -> CommandResult {
	msg.respond_str(ctx, generic::summon(ctx, msg.guild_id, msg.author.id).await)
		.await?;

	Ok(())
}

pub async fn summon_interaction(
	ctx: &Context,
	interaction: &ApplicationCommandInteraction,
) -> serenity::Result<()> {
	interaction
		.respond_str(
			&ctx,
			generic::summon(ctx, interaction.guild_id, interaction.user.id).await,
		)
		.await
}

pub fn summon_interaction_create(
	command: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
	command
		.name("summon")
		.description("Summon the bot to your current voice channel")
}

#[command]
#[only_in(guilds)]
#[help_available]
#[description("Remove the bot from the voice channel it is in")]
pub async fn banish(ctx: &Context, msg: &Message) -> CommandResult {
	msg.respond_str(ctx, generic::banish(ctx, msg.guild_id).await)
		.await?;

	Ok(())
}

pub async fn banish_interaction(
	ctx: &Context,
	interaction: &ApplicationCommandInteraction,
) -> serenity::Result<()> {
	interaction
		.respond_str(&ctx, generic::banish(ctx, interaction.guild_id).await)
		.await
}

pub fn banish_interaction_create(
	command: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
	command
		.name("banish")
		.description("Banish the bot from its current voice channel")
}

#[command]
#[only_in(guilds)]
#[help_available]
#[description("Play the specified clip immediately")]
#[num_args(1)]
#[usage("<clip>")]
#[example("bnw/needoffspring")]
pub async fn clip(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	let path = args.current();

	msg.respond_str(
		ctx,
		generic::play(ctx, PlayStyle::Clip, path, msg.guild_id).await,
	)
	.await?;

	Ok(())
}

pub async fn clip_interaction(
	ctx: &Context,
	interaction: &ApplicationCommandInteraction,
) -> serenity::Result<()> {
	let clip = interaction.data.options.iter().find_map(|option| {
		if option.name == "clip" {
			option.value.as_ref()
		} else {
			None
		}
	});

	let clip = match clip {
		Some(Value::String(clip)) => Some(clip.as_str()),
		None => None,
		Some(_) => {
			error!("Error in clip interaction expecting string argument");
			return interaction.respond_str(&ctx, "Internal bot error").await;
		}
	};

	interaction
		.respond_str(
			ctx,
			generic::play(ctx, PlayStyle::Clip, clip, interaction.guild_id).await,
		)
		.await
}

pub fn clip_interaction_create(
	command: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
	command
		.name("clip")
		.description("Play the specified clip immediately")
		.create_option(|option| {
			option
				.name("clip")
				.description("Clip to play")
				.kind(ApplicationCommandOptionType::String)
				.required(true)
		})
}

#[command]
#[aliases(q)]
#[only_in(guilds)]
#[help_available]
#[description("Add a youtube or spotify source to the queue")]
#[min_args(1)]
#[usage("<source>")]
#[example("https://www.youtube.com/watch?v=k2mFvwDTTt0")]
pub async fn play(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	let query = args.raw().join(" ");
	let query = if query.len() == 0 {
		None
	} else {
		Some(query.as_str())
	};

	msg.respond_str(
		ctx,
		generic::play(ctx, PlayStyle::Play, query, msg.guild_id).await,
	)
	.await?;

	Ok(())
}

pub async fn play_interaction(
	ctx: &Context,
	interaction: &ApplicationCommandInteraction,
) -> serenity::Result<()> {
	let clip = interaction.data.options.iter().find_map(|option| {
		if option.name == "input" {
			option.value.as_ref()
		} else {
			None
		}
	});

	let clip = match clip {
		Some(Value::String(clip)) => Some(clip.as_str()),
		None => None,
		Some(_) => {
			error!("Error in play interaction expecting string argument");
			return interaction.respond_str(&ctx, "Internal bot error").await;
		}
	};

	interaction
		.respond_str(
			ctx,
			generic::play(ctx, PlayStyle::Play, clip, interaction.guild_id).await,
		)
		.await
}

pub fn play_interaction_create(
	command: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
	command
		.name("play")
		.description("Add a youtube or spotify source to the queue")
		.create_option(|option| {
			option
				.name("input")
				.description("Youtube or Spotify URL, or youtube search")
				.kind(ApplicationCommandOptionType::String)
				.required(true)
		})
}

#[command]
#[only_in(guilds)]
#[help_available]
#[description("Change volume of bot")]
#[num_args(2)]
#[usage("<play|clip> <volume>")]
#[example("play 0.5")]
pub async fn volume(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
	let style = args
		.single::<PlayStyle>()
		.or_err_say(
			ctx,
			msg,
			"Expected either \"play\" or \"clip\" volume to be selected",
		)
		.await?;

	let volume = args
		.single::<f32>()
		.or_err_say(ctx, msg, "Volume must be a valid float between 0.0 and 1.0")
		.await?;

	msg.respond_str(
		ctx,
		generic::volume(ctx, Some(style), msg.guild_id, Some(volume)).await,
	)
	.await?;

	Ok(())
}

pub async fn volume_interaction(
	ctx: &Context,
	interaction: &ApplicationCommandInteraction,
) -> serenity::Result<()> {
	let style = interaction.data.options.iter().find_map(|option| {
		if option.name == "style" {
			option.value.as_ref()
		} else {
			None
		}
	});

	let style = match style {
		Some(Value::String(style)) => style.parse::<PlayStyle>().ok(),
		None => None,
		Some(_) => {
			error!("Error in volume interaction expecting float argument");
			return interaction.respond_str(&ctx, "Internal bot error").await;
		}
	};

	let volume = interaction.data.options.iter().find_map(|option| {
		if option.name == "volume" {
			option.value.as_ref()
		} else {
			None
		}
	});

	let volume = match volume {
		Some(Value::Number(volume)) => volume.as_f64().map(|v| v as f32),
		None => None,
		Some(_) => {
			error!("Error in volume interaction expecting float argument");
			return interaction.respond_str(&ctx, "Internal bot error").await;
		}
	};

	interaction
		.respond_str(
			ctx,
			generic::volume(ctx, style, interaction.guild_id, volume).await,
		)
		.await
}

pub fn volume_interaction_create(
	command: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
	command
		.name("volume")
		.description("Change volume of bot")
		.create_option(|option| {
			option
				.name("style")
				.description("Volume to set, either for play or clip commands")
				.kind(ApplicationCommandOptionType::String)
				.add_string_choice("play", "play")
				.add_string_choice("clip", "clip")
				.required(true)
		})
		.create_option(|option| {
			option
				.name("volume")
				.description("Volume between 0.0 and 1.0")
				.kind(ApplicationCommandOptionType::Number)
				.required(true)
		})
}

#[command]
#[only_in(guilds)]
#[help_available]
#[description("Stop all clips currently being played by the bot")]
pub async fn stop(ctx: &Context, msg: &Message) -> CommandResult {
	msg.respond_str(ctx, generic::stop(ctx, msg.guild_id).await)
		.await?;

	Ok(())
}

pub async fn stop_interaction(
	ctx: &Context,
	interaction: &ApplicationCommandInteraction,
) -> serenity::Result<()> {
	interaction
		.respond_str(ctx, generic::stop(ctx, interaction.guild_id).await)
		.await
}

pub fn stop_interaction_create(
	command: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
	command
		.name("stop")
		.description("Stop all clips currently being played by the bot")
}

#[command]
#[only_in(guilds)]
#[help_available]
#[description("Skip the current song in the queue")]
pub async fn skip(ctx: &Context, msg: &Message) -> CommandResult {
	msg.respond_str(ctx, generic::skip(ctx, msg.guild_id).await)
		.await?;

	Ok(())
}

pub async fn skip_interaction(
	ctx: &Context,
	interaction: &ApplicationCommandInteraction,
) -> serenity::Result<()> {
	interaction
		.respond_str(ctx, generic::skip(ctx, interaction.guild_id).await)
		.await
}

pub fn skip_interaction_create(
	command: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
	command
		.name("skip")
		.description("Skip the current song in the queue")
}

#[command]
#[help_available]
#[description("List all the sections and/or clips available in the section")]
#[min_args(0)]
#[max_args(1)]
#[usage("[section]")]
#[example("bnw")]
pub async fn list(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	if args.len() > 1 {
		msg.respond_str(ctx, "Expected at most one path to be specified")
			.await?;
		return Ok(());
	}

	let path = args.current();
	msg.respond_str(ctx, generic::list(path).await).await?;

	return Ok(());
}

pub async fn list_interaction(
	ctx: &Context,
	interaction: &ApplicationCommandInteraction,
) -> serenity::Result<()> {
	let path = interaction.data.options.iter().find_map(|option| {
		if option.name == "path" {
			option.value.as_ref()
		} else {
			None
		}
	});

	let path = match path {
		Some(Value::String(path)) => Some(path.as_str()),
		None => None,
		Some(_) => {
			error!("Error in list interaction expecting string argument");
			return interaction.respond_str(&ctx, "Internal bot error").await;
		}
	};

	interaction
		.respond_str(ctx, generic::list(path).await)
		.await
}

pub fn list_interaction_create(
	command: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
	command
		.name("list")
		.description("List all the sections and/or clips available in the section")
		.create_option(|option| {
			option
				.name("path")
				.description("Path to list clips underneath")
				.kind(ApplicationCommandOptionType::String)
		})
}
