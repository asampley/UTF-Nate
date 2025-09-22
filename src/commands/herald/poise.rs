use crate::commands::{CustomData, run};
use crate::util::*;

use super::IntroOutroMode::*;
use super::{IntroBotArgs, IntroOutroArgs};

#[poise::command(
	category = "herald",
	prefix_command,
	slash_command,
	custom_data = "CustomData::new(super::intro_help)"
)]
pub async fn intro(
	ctx: Context<'_>,
	#[description = "Clip search to play when you enter the channel"]
	#[rest]
	clip: Option<String>,
) -> CommandResult {
	run(
		&ctx,
		super::intro_outro(&ctx.into(), &(&ctx).into(), Intro, &IntroOutroArgs { clip }),
	)
	.await
}

#[poise::command(
	category = "herald",
	prefix_command,
	slash_command,
	guild_only,
	custom_data = "CustomData::new(super::introbot_help)"
)]
pub async fn introbot(
	ctx: Context<'_>,
	#[description = "Clip search to play when the bot enters a channel in this guild"]
	#[rest]
	clip: Option<String>,
) -> CommandResult {
	run(
		&ctx,
		super::introbot(&ctx.into(), &(&ctx).into(), &IntroBotArgs { clip }),
	)
	.await
}

#[poise::command(
	category = "herald",
	prefix_command,
	slash_command,
	custom_data = "CustomData::new(super::outro_help)"
)]
pub async fn outro(
	ctx: Context<'_>,
	#[description = "Clip search to play when you exit the channel"]
	#[rest]
	clip: Option<String>,
) -> CommandResult {
	run(
		&ctx,
		super::intro_outro(&ctx.into(), &(&ctx).into(), Outro, &IntroOutroArgs { clip }),
	)
	.await
}
