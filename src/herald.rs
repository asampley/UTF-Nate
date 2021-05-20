use serenity::client::Context;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;

use std::path::Path;

use crate::configuration;
use crate::configuration::write_config;
use crate::data::ConfigResource;
use crate::voice::get_clip;

#[group("herald")]
#[description("Commands to change intro and outro clips for each user")]
#[commands(intro, outro, introbot)]
pub struct Herald;

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

	let clip_str = args.current().unwrap();
	match get_clip(clip_str) {
		Some(_) => (),
		None => {
			msg.channel_id.say(&ctx.http, "Invalid clip").await?;
			return Ok(());
		}
	}

	let mut data_lock = ctx.data.write().await;
	let config_arc = data_lock
		.get_mut::<ConfigResource>()
		.expect("Expected ConfigResource in ShareMap")
		.clone();

	let mut config = config_arc.write().await;

	config.intros.insert(msg.author.id, clip_str.to_string());

	{
		use configuration::Result::*;
		match write_config(Path::new("config.json"), &*config) {
			Ok(()) => (),
			JsonError(reason) => eprintln!("Error writing config file: {:?}", reason),
			IoError(reason) => eprintln!("Error writing config file: {:?}", reason),
		}
	}

	msg.channel_id.say(&ctx.http, "Set new intro").await?;
	Ok(())
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

	let guild_id = match msg.guild_id {
		Some(guild_id) => guild_id,
		None => {
			msg.channel_id
				.say(&ctx.http, "Groups and DMs not supported")
				.await?;
			return Ok(());
		}
	};

	let clip_str = args.current().unwrap();
	match get_clip(clip_str) {
		Some(_) => (),
		None => {
			msg.channel_id.say(&ctx.http, "Invalid clip").await?;
			return Ok(());
		}
	}

	let mut data_lock = ctx.data.write().await;
	let config_arc = data_lock
		.get_mut::<ConfigResource>()
		.expect("Expected ConfigResource in ShareMap")
		.clone();

	let mut config = config_arc.write().await;

	config.guilds.entry(guild_id).or_default().bot_intro = Some(clip_str.to_string());

	{
		use configuration::Result::*;
		match write_config(Path::new("config.json"), &*config) {
			Ok(()) => (),
			JsonError(reason) => eprintln!("Error writing config file: {:?}", reason),
			IoError(reason) => eprintln!("Error writing config file: {:?}", reason),
		}
	}

	msg.channel_id.say(&ctx.http, "Set new intro").await?;
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

