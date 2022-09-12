use itertools::Itertools;

use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{Args, Command, CommandResult};
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::channel::Message;

use crate::audio::PlayStyle;
use crate::commands::{create_interaction, run};
use crate::util::*;

mod generic;

#[group("play")]
#[description("Play or queue sounds")]
#[commands(clip, play, playnext, playnow)]
pub struct Play;

#[command]
#[only_in(guilds)]
#[help_available]
#[description("Play the specified clip immediately")]
#[min_args(1)]
#[usage("<clip>")]
#[example("peon werk werk")]
#[example("peon/work work")]
pub async fn clip(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	let path = args.remains();

	run(
		ctx,
		msg,
		generic::play(ctx, PlayStyle::Clip, path, msg.guild_id, None),
	)
	.await
}

pub async fn clip_interaction(
	ctx: &Context,
	int: &ApplicationCommandInteraction,
) -> serenity::Result<()> {
	let clip = get_option_string(ctx, int, &int.data.options, "clip").await?;

	run(
		ctx,
		int,
		generic::play(ctx, PlayStyle::Clip, clip, int.guild_id, None),
	)
	.await
}

pub fn clip_interaction_create(
	cmd: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
	create_interaction(&CLIP_COMMAND, cmd).create_option(|option| {
		option
			.name("clip")
			.description("Clip to play")
			.kind(CommandOptionType::String)
			.required(true)
	})
}

async fn play_type_command(
	ctx: &Context,
	msg: &Message,
	args: Args,
	play_index: Option<usize>,
) -> CommandResult {
	let query = args.raw().join(" ");
	let query = if query.len() == 0 {
		None
	} else {
		Some(query.as_str())
	};

	run(
		ctx,
		msg,
		generic::play(ctx, PlayStyle::Play, query, msg.guild_id, play_index),
	)
	.await
}

async fn play_type_interaction(
	ctx: &Context,
	int: &ApplicationCommandInteraction,
	play_index: Option<usize>,
) -> serenity::Result<()> {
	let clip = get_option_string(ctx, int, &int.data.options, "input").await?;

	run(
		ctx,
		int,
		generic::play(ctx, PlayStyle::Play, clip, int.guild_id, play_index),
	)
	.await
}

fn play_type_interaction_create<'a>(
	cmd: &Command,
	create: &'a mut CreateApplicationCommand,
) -> &'a mut CreateApplicationCommand {
	create_interaction(cmd, create).create_option(|option| {
		option
			.name("input")
			.description("Youtube or Spotify URL, or youtube search")
			.kind(CommandOptionType::String)
			.required(true)
	})
}

#[command]
#[only_in(guilds)]
#[help_available]
#[description(
	"Add a youtube video, playlist, search, or spotify song, playlist, or album to the queue"
)]
#[min_args(1)]
#[usage("<source>")]
#[example("arbitrary youtube search")]
#[example("https://www.youtube.com/watch?v=k2mFvwDTTt0")]
#[example("https://www.youtube.com/playlist?list=PLucOLpdAYaKW1IYuo84R4qIskTfj-ECDp")]
#[example("https://open.spotify.com/track/009bpReJuXgCv8G2MkJ5Y1")]
#[example("https://open.spotify.com/album/0G2RxSCixG5Nl6jpjwiw2g")]
#[example("https://open.spotify.com/playlist/2O18dCV9uoGTyxN5HLJkTo")]
pub async fn play(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	play_type_command(ctx, msg, args, None).await
}

pub async fn play_interaction(
	ctx: &Context,
	int: &ApplicationCommandInteraction,
) -> serenity::Result<()> {
	play_type_interaction(ctx, int, None).await
}

pub fn play_interaction_create(
	cmd: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
	play_type_interaction_create(&PLAY_COMMAND, cmd)
}

#[command]
#[only_in(guilds)]
#[help_available]
#[description("Play after the previous item in the queue finishes")]
#[min_args(1)]
#[usage("<source>")]
#[example("arbitrary youtube search")]
#[example("https://www.youtube.com/watch?v=k2mFvwDTTt0")]
#[example("https://open.spotify.com/track/009bpReJuXgCv8G2MkJ5Y1")]
pub async fn playnext(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	play_type_command(ctx, msg, args, Some(1)).await
}

pub async fn playnext_interaction(
	ctx: &Context,
	int: &ApplicationCommandInteraction,
) -> serenity::Result<()> {
	play_type_interaction(ctx, int, Some(1)).await
}

pub fn playnext_interaction_create(
	cmd: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
	play_type_interaction_create(&PLAYNEXT_COMMAND, cmd)
}

#[command]
#[only_in(guilds)]
#[help_available]
#[description("Play immediately, delaying the previously playing item")]
#[min_args(1)]
#[usage("<source>")]
#[example("arbitrary youtube search")]
#[example("https://www.youtube.com/watch?v=k2mFvwDTTt0")]
#[example("https://open.spotify.com/track/009bpReJuXgCv8G2MkJ5Y1")]
pub async fn playnow(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	play_type_command(ctx, msg, args, Some(0)).await
}

pub async fn playnow_interaction(
	ctx: &Context,
	int: &ApplicationCommandInteraction,
) -> serenity::Result<()> {
	play_type_interaction(ctx, int, Some(0)).await
}

pub fn playnow_interaction_create(
	cmd: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
	play_type_interaction_create(&PLAYNOW_COMMAND, cmd)
}
