use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;
use serenity::model::interactions::application_command::{
	ApplicationCommandInteraction, ApplicationCommandOptionType,
};

use crate::commands::{create_interaction, run};
use crate::util::*;

mod generic;

pub enum IntroOutroMode {
	Intro,
	Outro,
}

impl IntroOutroMode {
	fn lowercase(&self) -> &'static str {
		match self {
			Intro => "intro",
			Outro => "outro",
		}
	}
}

use IntroOutroMode::*;

#[group("herald")]
#[description("Commands to change intro and outro clips for each user")]
#[commands(intro, outro, introbot)]
pub struct Herald;

pub async fn intro_outro_interaction(
	ctx: &Context,
	int: &ApplicationCommandInteraction,
	mode: IntroOutroMode,
) -> serenity::Result<()> {
	let clip = match get_option_string(ctx, int, &int.data.options, "clip").await {
		Ok(value) => value.map(|s| s.to_string()),
		Err(result) => return result,
	};

	run(
		ctx,
		int,
		generic::intro_outro(&ctx, mode, int.user.id, clip),
	)
	.await
}

pub fn intro_interaction_create(
	cmd: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
	create_interaction(&INTRO_COMMAND, cmd).create_option(|option| {
		option
			.name("clip")
			.description("Clip path to play when you enter a channel")
			.kind(ApplicationCommandOptionType::String)
	})
}

pub fn outro_interaction_create(
	cmd: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
	create_interaction(&OUTRO_COMMAND, cmd).create_option(|option| {
		option
			.name("clip")
			.description("Clip path to play when you exit a channel")
			.kind(ApplicationCommandOptionType::String)
	})
}

#[command]
#[help_available]
#[description("Set the clip to be played when you enter the channel containing the bot")]
#[max_args(1)]
#[usage("<clip>")]
#[example("")]
#[example("angels")]
#[example("bnw/angels")]
pub async fn intro(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	let clip = args.current().map(|s| s.to_string());

	run(
		ctx,
		msg,
		generic::intro_outro(&ctx, Intro, msg.author.id, clip),
	)
	.await
}

pub async fn introbot_interaction(
	ctx: &Context,
	int: &ApplicationCommandInteraction,
) -> serenity::Result<()> {
	let clip = match get_option_string(ctx, int, &int.data.options, "clip").await {
		Ok(value) => value.map(|s| s.to_string()),
		Err(result) => return result,
	};

	run(ctx, int, generic::introbot(&ctx, int.guild_id, clip)).await
}

pub fn introbot_interaction_create(
	cmd: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
	create_interaction(&INTROBOT_COMMAND, cmd).create_option(|option| {
		option
			.name("clip")
			.description("Clip path to play when the bot enters a channel in this guild")
			.kind(ApplicationCommandOptionType::String)
	})
}

#[command]
#[only_in(guilds)]
#[help_available]
#[description("Set the clip to be played when the bot enters a channel")]
#[max_args(1)]
#[usage("<clip>")]
#[example("")]
#[example("angels")]
#[example("bnw/angels")]
pub async fn introbot(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	let clip_str = args.current().map(|s| s.to_string());

	run(ctx, msg, generic::introbot(&ctx, msg.guild_id, clip_str)).await
}

#[command]
#[help_available]
#[description("Set the clip to be played when you exit the channel containing the bot")]
#[max_args(1)]
#[usage("<clip>")]
#[example("")]
#[example("death")]
#[example("bnw/death")]
pub async fn outro(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	let clip = args.current().map(|s| s.to_string());

	run(
		ctx,
		msg,
		generic::intro_outro(&ctx, Outro, msg.author.id, clip),
	)
	.await
}
