use crate::audio::PlayStyle;
use crate::commands::{CustomData, run};
use crate::util::*;

use super::PlayArgs;

#[poise::command(
	category = "play",
	prefix_command,
	slash_command,
	guild_only,
	custom_data = "CustomData::new(super::clip_help)"
)]
pub async fn clip(
	ctx: Context<'_>,
	#[description = "Clip to play"]
	#[rest]
	clip: String,
) -> CommandResult {
	run(
		&ctx,
		super::play(
			&ctx.into(),
			&(&ctx).into(),
			PlayStyle::Clip,
			None,
			&PlayArgs { search: clip },
		),
	)
	.await
}

async fn play_type_command(
	ctx: Context<'_>,
	search: String,
	play_index: Option<usize>,
) -> CommandResult {
	run(
		&ctx,
		super::play(
			&ctx.into(),
			&(&ctx).into(),
			PlayStyle::Play,
			play_index,
			&PlayArgs { search },
		),
	)
	.await
}

#[poise::command(
	category = "play",
	prefix_command,
	slash_command,
	guild_only,
	custom_data = "CustomData::new(super::play_help)"
)]
pub async fn play(
	ctx: Context<'_>,
	#[description = "Youtube or Spotify URL, or Youtube search"]
	#[rest]
	query: String,
) -> CommandResult {
	play_type_command(ctx, query, None).await
}

#[poise::command(
	category = "play",
	prefix_command,
	slash_command,
	guild_only,
	custom_data = "CustomData::new(super::playnext_help)"
)]
pub async fn playnext(
	ctx: Context<'_>,
	#[description = "Youtube or Spotify URL, or Youtube search"]
	#[rest]
	query: String,
) -> CommandResult {
	play_type_command(ctx, query, Some(1)).await
}

#[poise::command(
	category = "play",
	prefix_command,
	slash_command,
	guild_only,
	custom_data = "CustomData::new(super::playnow_help)"
)]
pub async fn playnow(
	ctx: Context<'_>,
	#[description = "Youtube or Spotify URL, or Youtube search"]
	#[rest]
	query: String,
) -> CommandResult {
	play_type_command(ctx, query, Some(0)).await
}
