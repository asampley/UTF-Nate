use crate::audio::PlayStyle;
use crate::commands::run;
use crate::util::*;

mod generic;

/// Get or change the volume of the bot
///
/// **Usage:** `volume <get|play|clip> <volume?>`
///
/// **Examples:**
/// - `volume get`
/// - `volume play`
/// - `volume clip`
/// - `volume play .25`
/// - `volume clip 0.5`
#[poise::command(
	category = "voice",
	prefix_command,
	slash_command,
	guild_only,
	subcommands("volume_get", "volume_play", "volume_clip")
)]
pub async fn volume(ctx: Context<'_>) -> CommandResult {
	run(&ctx, generic::volume(&ctx, None, ctx.guild_id(), None)).await
}

/// Get or change the volume of the bot
///
/// **Usage:** `volume get`
#[poise::command(
	category = "voice",
	rename = "get",
	prefix_command,
	slash_command,
	guild_only
)]
pub async fn volume_get(ctx: Context<'_>) -> CommandResult {
	run(&ctx, generic::volume(&ctx, None, ctx.guild_id(), None)).await
}

/// Get or change the volume of the bot
///
/// **Usage:** `volume play <volume?>`
///
/// **Examples:**
/// - `volume play`
/// - `volume play .25`
/// - `volume play 0.5`
#[poise::command(
	category = "voice",
	rename = "play",
	prefix_command,
	slash_command,
	guild_only
)]
pub async fn volume_play(
	ctx: Context<'_>,
	#[description = "Volume between 0.0 and 1.0"] volume: Option<f32>,
) -> CommandResult {
	run(
		&ctx,
		generic::volume(&ctx, Some(PlayStyle::Play), ctx.guild_id(), volume),
	)
	.await
}

/// Get or change the volume of the bot
///
/// **Usage:** `volume clip <volume?>`
///
/// **Examples:**
/// - `volume clip`
/// - `volume clip .25`
/// - `volume clip 0.5`
#[poise::command(
	category = "voice",
	rename = "clip",
	prefix_command,
	slash_command,
	guild_only
)]
pub async fn volume_clip(
	ctx: Context<'_>,
	#[description = "Volume between 0.0 and 1.0"] volume: Option<f32>,
) -> CommandResult {
	run(
		&ctx,
		generic::volume(&ctx, Some(PlayStyle::Clip), ctx.guild_id(), volume),
	)
	.await
}

/// List all the sections and/or clips available in the section
///
/// **Usage:** `list <section?>`
///
/// **Examples:**
/// - `list bnw`
#[poise::command(category = "voice", prefix_command, slash_command)]
pub async fn list(
	ctx: Context<'_>,
	#[description = "Path to list clips underneath"] path: Option<String>,
) -> CommandResult {
	run(&ctx, generic::list(path.as_deref())).await
}
