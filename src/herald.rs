use serde_json::value::Value;

use serenity::client::Context;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;
use serenity::model::prelude::{UserId, GuildId};
use serenity::model::interactions::application_command::ApplicationCommandInteraction;

use std::path::Path;

use crate::configuration;
use crate::configuration::write_config;
use crate::data::ConfigResource;
use crate::util::Respond;
use crate::voice::get_clip;

pub enum IntroOutroMode {
	Intro,
	Outro,
}

use IntroOutroMode::*;

#[group("herald")]
#[description("Commands to change intro and outro clips for each user")]
#[commands(intro, outro, introbot)]
pub struct Herald;

pub async fn intro_outro_generic(
	ctx: &Context,
	mode: IntroOutroMode,
	user_id: UserId,
	clip: Option<String>,
) -> String {
	let clip = match clip {
		Some(clip) => clip,
		None => return "No clip provided".to_string(),
	};

	match get_clip(&clip) {
		Some(_) => (),
		None => return format!("Invalid clip: {}", clip),
	}

	let mut data_lock = ctx.data.write().await;
	let config_arc = data_lock
		.get_mut::<ConfigResource>()
		.expect("Expected ConfigResource in ShareMap")
		.clone();

	let mut config = config_arc.write().await;

	match mode {
		Intro => config.intros.insert(user_id, clip.clone()),
		Outro => config.outros.insert(user_id, clip.clone()),
	};

	{
		use configuration::Result::*;
		match write_config(Path::new("config.json"), &*config) {
			Ok(()) => (),
			JsonError(reason) => eprintln!("Error writing config file: {:?}", reason),
			IoError(reason) => eprintln!("Error writing config file: {:?}", reason),
		}
	}

	format!("Set new {} to {}", match mode { Intro => "intro", Outro => "outro" }, clip)
}

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

	interaction.respond_str(&ctx, intro_outro_generic(&ctx, mode, interaction.user.id, clip).await).await
}

#[command]
#[help_available]
#[description("Set the clip to be played when you enter the channel containing the bot")]
#[num_args(1)]
#[usage("<clip>")]
#[example("bnw/angels")]
pub async fn intro(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	if args.len() != 1 {
		msg.channel_id
			.say(&ctx.http, "Expected exactly one clip")
			.await?;
		return Ok(());
	}

	let clip = args.current().map(|s| s.to_string());

	msg.channel_id.say(&ctx.http, intro_outro_generic(&ctx, Intro, msg.author.id, clip).await).await?;

	Ok(())
}

pub async fn introbot_generic(ctx: &Context, guild_id: Option<GuildId>, clip: Option<String>) -> String {
	let guild_id = match guild_id {
		Some(guild_id) => guild_id,
		None => return "Groups and DMs not supported".to_string(),
	};

	let clip = match clip {
		Some(clip) => clip,
		None => return "No clip provided".to_string(),
	};

	match get_clip(&clip) {
		Some(_) => (),
		None => return format!("Invalid clip: {}", clip),
	}

	let mut data_lock = ctx.data.write().await;
	let config_arc = data_lock
		.get_mut::<ConfigResource>()
		.expect("Expected ConfigResource in ShareMap")
		.clone();

	let mut config = config_arc.write().await;

	config.guilds.entry(guild_id).or_default().bot_intro = Some(clip.clone());

	{
		use configuration::Result::*;
		match write_config(Path::new("config.json"), &*config) {
			Ok(()) => (),
			JsonError(reason) => eprintln!("Error writing config file: {:?}", reason),
			IoError(reason) => eprintln!("Error writing config file: {:?}", reason),
		}
	}

	format!("Set bot intro to {}", clip)
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

	interaction.respond_str(&ctx, introbot_generic(&ctx, interaction.guild_id, clip).await).await
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
		msg.channel_id
			.say(&ctx.http, "Expected exactly one clip")
			.await?;
		return Ok(());
	}

	let clip_str = args.current().map(|s| s.to_string());

	msg.channel_id.say(&ctx.http, introbot_generic(&ctx, msg.guild_id, clip_str).await).await?;

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
		msg.channel_id
			.say(&ctx.http, "Expected exactly one clip")
			.await?;
		return Ok(());
	}

	let clip_str = args.current().unwrap();
	match get_clip(clip_str) {
		Some(_) => (),
		None => {
			msg.channel_id.say(&ctx.http, "Invalid clip").await?;
			return Ok(());
		}
	};

	let mut data_lock = ctx.data.write().await;
	let config_arc = data_lock
		.get_mut::<ConfigResource>()
		.expect("Expected ConfigResource in ShareMap")
		.clone();

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

	msg.channel_id.say(&ctx.http, "Set new outro").await?;
	Ok(())
}

