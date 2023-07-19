use axum::extract::{Query, State};
use axum::response::Html;

use axum_extra::extract::CookieJar;

use crate::audio::PlayStyle;
use crate::commands::http::{extract_source, response_to_html_string, run};
use crate::commands::BotState;
use crate::util::GetExpect;
use crate::AeadKey;

use super::{VolumeMode, VolumeSetArgs};

pub async fn volume_get(State(state): State<BotState>, jar: CookieJar) -> Html<String> {
	let source = match extract_source(&jar, state.data.read().await.get_expect::<AeadKey>()) {
		Err(e) => return Html(response_to_html_string(Err(e))),
		Ok(source) => source,
	};

	run(
		|_| super::volume(&state, &source, VolumeMode::ConfigAllStyles),
		(),
	)
	.await
}

pub async fn volume_play(
	State(state): State<BotState>,
	jar: CookieJar,
	Query(args): Query<VolumeSetArgs>,
) -> Html<String> {
	let source = match extract_source(&jar, state.data.read().await.get_expect::<AeadKey>()) {
		Err(e) => return Html(response_to_html_string(Err(e))),
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
		&args,
	)
	.await
}

pub async fn volume_clip(
	State(state): State<BotState>,
	jar: CookieJar,
	Query(args): Query<VolumeSetArgs>,
) -> Html<String> {
	let source = match extract_source(&jar, state.data.read().await.get_expect::<AeadKey>()) {
		Err(e) => return Html(response_to_html_string(Err(e))),
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
		&args,
	)
	.await
}

pub async fn volume_now(
	State(state): State<BotState>,
	jar: CookieJar,
	Query(args): Query<VolumeSetArgs>,
) -> Html<String> {
	let source = match extract_source(&jar, state.data.read().await.get_expect::<AeadKey>()) {
		Err(e) => return Html(response_to_html_string(Err(e))),
		Ok(source) => source,
	};

	run(
		|a| super::volume(&state, &source, VolumeMode::Current(a.volume)),
		&args,
	)
	.await
}
