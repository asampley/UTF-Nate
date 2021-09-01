use serde_json::value::Value;

use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;
use serenity::model::interactions::application_command::{
	ApplicationCommandInteraction,
	ApplicationCommandOptionType,
};

use std::path::Path;

use crate::configuration;
use crate::configuration::write_config;
use crate::configuration::Config;
use crate::util::{GetExpect, Respond};
use crate::voice::get_clip;

mod generic;

pub enum IntroOutroMode {
	Intro,
	Outro,
}

use IntroOutroMode::*;

#[group("herald")]
#[description("Commands to change intro and outro clips for each user")]
#[commands(intro, outro, introbot)]
pub struct Herald;

pub async fn intro_outro_interaction(
	ctx: &Context,
	interaction: &ApplicationCommandInteraction,
	mode: IntroOutroMode,
) -> serenity::Result<()> {
	let clip = interaction.data.options.iter()
		.find_map(|option| if option.name == "clip" { option.value.as_ref() } else { None });

	let clip = match clip {
		Some(Value::String(clip)) => Some(clip.clone()),
		None => None,
		Some(_) => {
			eprintln!("Error in intro interaction expecting string argument");
			return interaction.respond_str(&ctx, "Internal bot error").await;
		}
	};

	interaction.respond_str(&ctx, generic::intro_outro(&ctx, mode, interaction.user.id, clip).await).await
}

pub fn intro_interaction_create(
	command: &mut CreateApplicationCommand
) -> &mut CreateApplicationCommand {
	command.name("intro")
		.description("Set the clip to be played when you enter the channel containing the bot")
		.create_option(|option|
			option
				.name("clip")
				.description("Clip path to play when you enter a channel")
				.kind(ApplicationCommandOptionType::String)
				.required(true)
		)
}

pub fn outro_interaction_create(
	command: &mut CreateApplicationCommand
) -> &mut CreateApplicationCommand {
	command.name("outro")
		.description("Set the clip to be played when you exit the channel containing the bot")
		.create_option(|option|
			option
				.name("clip")
				.description("Clip path to play when you exit a channel")
				.kind(ApplicationCommandOptionType::String)
				.required(true)
		)
}

#[command]
#[help_available]
#[description("Set the clip to be played when you enter the channel containing the bot")]
#[num_args(1)]
#[usage("<clip>")]
#[example("bnw/angels")]
pub async fn intro(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	if args.len() != 1 {
		msg.respond_str(ctx, "Expected exactly one clip").await?;
		return Ok(());
	}

	let clip = args.current().map(|s| s.to_string());

	msg.respond_str(ctx, generic::intro_outro(&ctx, Intro, msg.author.id, clip).await).await?;

	Ok(())
}

pub async fn introbot_interaction(
	ctx: &Context,
	interaction: &ApplicationCommandInteraction,
) -> serenity::Result<()> {
	let clip = interaction.data.options.iter()
		.find_map(|option| if option.name == "clip" { option.value.as_ref() } else { None });

	let clip = match clip {
		Some(Value::String(clip)) => Some(clip.clone()),
		None => None,
		Some(_) => {
			eprintln!("Error in intro interaction expecting string argument");
			return interaction.respond_str(&ctx, "Internal bot error").await;
		}
	};

	interaction.respond_str(&ctx, generic::introbot(&ctx, interaction.guild_id, clip).await).await
}

pub fn introbot_interaction_create(
	command: &mut CreateApplicationCommand
) -> &mut CreateApplicationCommand {
	command.name("introbot")
		.description("Set the clip to be played when the bot enters a channel in this guild")
		.create_option(|option|
			option
				.name("clip")
				.description("Clip path to play when the bot enters a channel in this guild")
				.kind(ApplicationCommandOptionType::String)
				.required(true)
		)
}

#[command]
#[only_in(guilds)]
#[help_available]
#[description("Set the clip to be played when you enter the channel containing the bot")]
#[num_args(1)]
#[usage("<clip>")]
#[example("bnw/angels")]
pub async fn introbot(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	if args.len() != 1 {
		msg.respond_str(ctx, "Expected exactly one clip").await?;
		return Ok(());
	}

	let clip_str = args.current().map(|s| s.to_string());

	msg.respond_str(ctx, generic::introbot(&ctx, msg.guild_id, clip_str).await).await?;

	Ok(())
} 

#[command]
#[help_available]
#[description("Set the clip to be played when you exit the channel containing the bot")]
#[num_args(1)]
#[usage("<clip>")]
#[example("bnw/death")]
pub async fn outro(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	if args.len() != 1 {
		msg.respond_str(ctx, "Expected exactly one clip").await?;
		return Ok(());
	}

	let clip_str = args.current().unwrap();
	match get_clip(clip_str) {
		Some(_) => (),
		None => {
			msg.respond_str(ctx, "Invalid clip").await?;
			return Ok(());
		}
	};

	let data_lock = ctx.data.write().await;
	let config_arc = data_lock.clone_expect::<Config>();

	let mut config = config_arc.write().await;

	config.outros.insert(msg.author.id, clip_str.to_string());

	{
		use configuration::Result::*;
		match write_config(Path::new("config.json"), &*config) {
			Ok(()) => (),
			JsonError(reason) => eprintln!("Error writing config file: {:?}", reason),
			IoError(reason) => eprintln!("Error writing config file: {:?}", reason),
		}
	}

	msg.respond_str(ctx, "Set new outro").await?;
	Ok(())
}
