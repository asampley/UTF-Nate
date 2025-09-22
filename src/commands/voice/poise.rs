use crate::audio::PlayStyle;
use crate::commands::{CustomData, run};
use crate::util::*;

use super::VolumeMode;

#[poise::command(
	category = "voice",
	prefix_command,
	slash_command,
	guild_only,
	subcommands("volume_get", "volume_play", "volume_clip", "volume_now"),
	custom_data = "CustomData::new(super::volume_help)"
)]
pub async fn volume(ctx: Context<'_>) -> CommandResult {
	run(
		&ctx,
		super::volume(&ctx.into(), &(&ctx).into(), VolumeMode::ConfigAllStyles),
	)
	.await
}

#[poise::command(
	category = "voice",
	rename = "get",
	prefix_command,
	slash_command,
	guild_only,
	custom_data = "CustomData::new(super::volume_get_help)"
)]
pub async fn volume_get(ctx: Context<'_>) -> CommandResult {
	run(
		&ctx,
		super::volume(&ctx.into(), &(&ctx).into(), VolumeMode::ConfigAllStyles),
	)
	.await
}

#[poise::command(
	category = "voice",
	rename = "play",
	prefix_command,
	slash_command,
	guild_only,
	custom_data = "CustomData::new(super::volume_play_help)"
)]
pub async fn volume_play(
	ctx: Context<'_>,
	#[description = "Volume between 0.0 and 1.0"] volume: Option<f32>,
) -> CommandResult {
	run(
		&ctx,
		super::volume(
			&ctx.into(),
			&(&ctx).into(),
			VolumeMode::Config(PlayStyle::Play, volume),
		),
	)
	.await
}

#[poise::command(
	category = "voice",
	rename = "clip",
	prefix_command,
	slash_command,
	guild_only,
	custom_data = "CustomData::new(super::volume_clip_help)"
)]
pub async fn volume_clip(
	ctx: Context<'_>,
	#[description = "Volume between 0.0 and 1.0"] volume: Option<f32>,
) -> CommandResult {
	run(
		&ctx,
		super::volume(
			&ctx.into(),
			&(&ctx).into(),
			VolumeMode::Config(PlayStyle::Clip, volume),
		),
	)
	.await
}

#[poise::command(
	category = "voice",
	rename = "now",
	prefix_command,
	slash_command,
	guild_only,
	custom_data = "CustomData::new(super::volume_now_help)"
)]
pub async fn volume_now(
	ctx: Context<'_>,
	#[description = "Volume between 0.0 and 1.0"] volume: Option<f32>,
) -> CommandResult {
	run(
		&ctx,
		super::volume(&ctx.into(), &(&ctx).into(), VolumeMode::Current(volume)),
	)
	.await
}

#[poise::command(
	category = "voice",
	prefix_command,
	slash_command,
	custom_data = "CustomData::new(super::list_help)"
)]
pub async fn list(
	ctx: Context<'_>,
	#[description = "Path to list clips underneath"] path: Option<String>,
) -> CommandResult {
	run(&ctx, super::list(path.as_deref())).await
}
