use axum::extract::{Query, State};
use axum::response::Html;

use axum_extra::extract::CookieJar;

use crate::audio::PlayStyle;
use crate::commands::http::{extract_source, response_to_html, run};
use crate::commands::BotState;

use super::{VolumeMode, VolumeSetArgs};

pub async fn volume_get(State(state): State<BotState>, jar: CookieJar) -> Html<String> {
	let source = match extract_source(&jar) {
		Err(e) => return Html(response_to_html(Err(e)).to_string()),
		Ok(source) => source,
	};

	run(
		|_| super::volume(&state, &source, VolumeMode::ConfigAllStyles),
		super::poise::volume_get,
		super::volume_get_help(),
		Some(()),
	)
	.await
}

pub async fn volume_play(
	State(state): State<BotState>,
	jar: CookieJar,
	query: Option<Query<VolumeSetArgs>>,
) -> Html<String> {
	let source = match extract_source(&jar) {
		Err(e) => return Html(response_to_html(Err(e)).to_string()),
		Ok(source) => source,
	};

	run(
		|a| {
			super::volume(
				&state,
				&source,
				VolumeMode::Config(PlayStyle::Play, a.volume),
			)
		},
		super::poise::volume_play,
		super::volume_play_help(),
		query.map(|q| q.0).as_ref(),
	)
	.await
}

pub async fn volume_clip(
	State(state): State<BotState>,
	jar: CookieJar,
	query: Option<Query<VolumeSetArgs>>,
) -> Html<String> {
	let source = match extract_source(&jar) {
		Err(e) => return Html(response_to_html(Err(e)).to_string()),
		Ok(source) => source,
	};

	run(
		|a| {
			super::volume(
				&state,
				&source,
				VolumeMode::Config(PlayStyle::Clip, a.volume),
			)
		},
		super::poise::volume_clip,
		super::volume_clip_help(),
		query.map(|q| q.0).as_ref(),
	)
	.await
}

pub async fn volume_now(
	State(state): State<BotState>,
	jar: CookieJar,
	query: Option<Query<VolumeSetArgs>>,
) -> Html<String> {
	let source = match extract_source(&jar) {
		Err(e) => return Html(response_to_html(Err(e)).to_string()),
		Ok(source) => source,
	};

	run(
		|a| super::volume(&state, &source, VolumeMode::Current(a.volume)),
		super::poise::volume_clip,
		super::volume_clip_help(),
		query.map(|q| q.0).as_ref(),
	)
	.await
}
