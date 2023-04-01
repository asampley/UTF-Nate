use crate::audio::PlayStyle;
use crate::commands::run;
use crate::util::*;

mod generic;

/// Play the specified clip immediately
///
/// **Usage:** `clip <clip>`
///
/// **Examples:**
/// - `clip peon werk werk`
/// - `clip peon/work work`
#[poise::command(category = "play", prefix_command, slash_command, guild_only)]
pub async fn clip(
	ctx: Context<'_>,
	#[description = "Clip to play"]
	#[rest]
	clip: String,
) -> CommandResult {
	run(
		&ctx,
		generic::play(ctx.discord(), PlayStyle::Clip, &clip, ctx.guild_id(), None),
	)
	.await
}

async fn play_type_command(
	ctx: Context<'_>,
	query: &str,
	play_index: Option<usize>,
) -> CommandResult {
	run(
		&ctx,
		generic::play(
			ctx.discord(),
			PlayStyle::Play,
			query,
			ctx.guild_id(),
			play_index,
		),
	)
	.await
}

/// Add a youtube video, playlist, search, or spotify song, playlist, or album to the queue
///
/// **Usage:** `play <source>`
///
/// **Examples:**
/// - `play arbitrary youtube search`
/// - `play https://www.youtube.com/watch?v=k2mFvwDTTt0`
/// - `play https://www.youtube.com/playlist?list=PLucOLpdAYaKW1IYuo84R4qIskTfj-ECDp`
/// - `play https://open.spotify.com/track/009bpReJuXgCv8G2MkJ5Y1`
/// - `play https://open.spotify.com/album/0G2RxSCixG5Nl6jpjwiw2g`
/// - `play https://open.spotify.com/playlist/2O18dCV9uoGTyxN5HLJkTo`
#[poise::command(category = "play", prefix_command, slash_command, guild_only)]
pub async fn play(
	ctx: Context<'_>,
	#[description = "Youtube or Spotify URL, or Youtube search"]
	#[rest]
	query: String,
) -> CommandResult {
	play_type_command(ctx, &query, None).await
}

/// Play after the previous item in the queue finishes
///
/// **Usage:** `playnext <source>`
///
/// **Examples:**
/// - `playnext arbitrary youtube search`
/// - `playnext https://www.youtube.com/watch?v=k2mFvwDTTt0`
/// - `playnext https://open.spotify.com/track/009bpReJuXgCv8G2MkJ5Y1`
#[poise::command(category = "play", prefix_command, slash_command, guild_only)]
pub async fn playnext(
	ctx: Context<'_>,
	#[description = "Youtube or Spotify URL, or Youtube search"]
	#[rest]
	query: String,
) -> CommandResult {
	play_type_command(ctx, &query, Some(1)).await
}

/// Play immediately, delaying the previously playing item
///
/// **Usage:** `<source>`
///
/// **Examples:**
/// - `arbitrary youtube search`
/// - `https://www.youtube.com/watch?v=k2mFvwDTTt0`
/// - `https://open.spotify.com/track/009bpReJuXgCv8G2MkJ5Y1`
#[poise::command(category = "play", prefix_command, slash_command, guild_only)]
pub async fn playnow(
	ctx: Context<'_>,
	#[description = "Youtube or Spotify URL, or Youtube search"]
	#[rest]
	query: String,
) -> CommandResult {
	play_type_command(ctx, &query, Some(0)).await
}

/// Generate audio using GPT based on a query
///
/// **Usage:** `gpt <prompt>`
///
/// **Examples:**
/// - `gpt prompt to generate audio`
#[poise::command(category = "ai", prefix_command, slash_command, guild_only)]
pub async fn gpt(
	ctx: Context<'_>,
	#[description = "GPT prompt"]
	#[rest]
	query: String,
) -> CommandResult {
	play_type_command(ctx, &query, None).await
}

/// Generate audio using GPT and put it next in the queue.
///
/// **Usage:** `gptnext <prompt>`
///
/// **Examples:**
/// - `gptnext prompt to generate audio`
#[poise::command(category = "ai", prefix_command, slash_command, guild_only)]
pub async fn gptnext(
	ctx: Context<'_>,
	#[description = "GPT prompt"]
	#[rest]
	query: String,
) -> CommandResult {
	play_type_command(ctx, &query, Some(1)).await
}

/// Play immediately, delaying the previously playing item
///
/// **Usage:** `gptnow <prompt>`
///
/// **Examples:**
/// - `gptnow prompt to generate audio`
#[poise::command(category = "ai", prefix_command, slash_command, guild_only)]
pub async fn gptnow(
	ctx: Context<'_>,
	#[description = "GPT prompt"]
	#[rest]
	query: String,
) -> CommandResult {
	play_type_command(ctx, &query, Some(0)).await
}
