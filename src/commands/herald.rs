use crate::commands::run;
use crate::util::*;

mod generic;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

/// Set the clip to be played when you enter the channel containing the bot
///
/// **Usage:** `intro <clip>`
///
/// **Examples:**
/// - `intro`
/// - `intro angels`
/// - `intro bnw/angels`
#[poise::command(category = "herald", prefix_command, slash_command, guild_only)]
pub async fn intro(
	ctx: Context<'_>,
	#[description = "Clip search to play when you enter the channel"]
	#[rest]
	clip: Option<String>,
) -> CommandResult {
	run(
		&ctx,
		generic::intro_outro(ctx.discord(), Intro, ctx.author().id, clip),
	)
	.await
}

/// Set the clip to be played when the bot enters a channel
///
/// **Usage:** `introbot <clip>`
///
/// **Examples:**
/// - `introbot`
/// - `introbot angels`
/// - `introbot bnw/angels`
#[poise::command(category = "herald", prefix_command, slash_command, guild_only)]
pub async fn introbot(
	ctx: Context<'_>,
	#[description = "Clip search to play when the bot enters a channel in this guild"]
	#[rest]
	clip: Option<String>,
) -> CommandResult {
	run(&ctx, generic::introbot(ctx.discord(), ctx.guild_id(), clip)).await
}

/// Set the clip to be played when you exit the channel containing the bot
///
/// **Usage:** `outro <clip>`
///
/// **Examples:**
/// - `outro`
/// - `outro death`
/// - `outro bnw/death`
#[poise::command(category = "herald", prefix_command, slash_command, guild_only)]
pub async fn outro(
	ctx: Context<'_>,
	#[description = "Clip search to play when you exit the channel"]
	#[rest]
	clip: Option<String>,
) -> CommandResult {
	run(
		&ctx,
		generic::intro_outro(ctx.discord(), Outro, ctx.author().id, clip),
	)
	.await
}
