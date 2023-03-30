use crate::audio::PlayStyle;
use crate::commands::run;
use crate::util::*;

mod generic;

use generic::VolumeMode;

/// Get or change the volume of the bot
///
/// **Usage:** `volume <get|play|clip|now> <volume?>`
///
/// **Examples:**
/// - `volume get`
/// - `volume play`
/// - `volume play .25`
/// - `volume clip`
/// - `volume clip 0.5`
/// - `volume now`
/// - `volume now 1.0`
#[poise::command(
	category = "voice",
	prefix_command,
	slash_command,
	guild_only,
	subcommands("volume_get", "volume_play", "volume_clip", "volume_now")
)]
pub async fn volume(ctx: Context<'_>) -> CommandResult {
	run(
		&ctx,
		generic::volume(&ctx, ctx.guild_id(), VolumeMode::ConfigAllStyles),
	)
	.await
}

/// Get the play and clip volumes of the bot
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
	run(
		&ctx,
		generic::volume(&ctx, ctx.guild_id(), VolumeMode::ConfigAllStyles),
	)
	.await
}

/// Get or change the play volume of the bot
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
	#[description = "Volume between 0.0 and 128.0"] volume: Option<f32>,
) -> CommandResult {
	run(
		&ctx,
		generic::volume(
			&ctx,
			ctx.guild_id(),
			VolumeMode::Config(PlayStyle::Play, volume),
		),
	)
	.await
}

/// Get or change the clip volume of the bot
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
		generic::volume(
			&ctx,
			ctx.guild_id(),
			VolumeMode::Config(PlayStyle::Clip, volume),
		),
	)
	.await
}

/// Get or change the play volume of the bot for the current song only
///
/// **Usage:** `volume now <volume?>`
///
/// **Examples:**
/// - `volume now`
/// - `volume now .25`
/// - `volume now 0.5`
#[poise::command(
	category = "voice",
	rename = "now",
	prefix_command,
	slash_command,
	guild_only
)]
pub async fn volume_now(
	ctx: Context<'_>,
	#[description = "Volume between 0.0 and 1.0"] volume: Option<f32>,
) -> CommandResult {
	run(
		&ctx,
		generic::volume(&ctx, ctx.guild_id(), VolumeMode::Current(volume)),
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
