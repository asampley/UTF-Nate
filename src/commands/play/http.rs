use axum::extract::{Query, State};
use axum::response::Html;

use axum_extra::extract::CookieJar;

use crate::commands::http::{extract_source, response_to_html_string, run};
use crate::commands::BotState;
use crate::util::GetExpect;
use crate::AeadKey;

use super::{PlayArgs, PlayStyle};

pub async fn clip(
	State(state): State<BotState>,
	jar: CookieJar,
	query: Option<Query<PlayArgs>>,
) -> Html<String> {
	let source = match extract_source(&jar, state.data.read().await.get_expect::<AeadKey>()) {
		Err(e) => return Html(response_to_html_string(Err(e))),
		Ok(source) => source,
	};

	run(
		|a| super::play(&state, &source, PlayStyle::Clip, None, a),
		super::poise::clip,
		super::clip_help(),
		query.map(|q| q.0).as_ref(),
	)
	.await
}

pub async fn play(
	State(state): State<BotState>,
	jar: CookieJar,
	query: Option<Query<PlayArgs>>,
) -> Html<String> {
	let source = match extract_source(&jar, state.data.read().await.get_expect::<AeadKey>()) {
		Err(e) => return Html(response_to_html_string(Err(e))),
		Ok(source) => source,
	};

	run(
		|a| super::play(&state, &source, PlayStyle::Play, None, a),
		super::poise::play,
		super::play_help(),
		query.map(|q| q.0).as_ref(),
	)
	.await
}

pub async fn playnext(
	State(state): State<BotState>,
	jar: CookieJar,
	query: Option<Query<PlayArgs>>,
) -> Html<String> {
	let source = match extract_source(&jar, state.data.read().await.get_expect::<AeadKey>()) {
		Err(e) => return Html(response_to_html_string(Err(e))),
		Ok(source) => source,
	};

	run(
		|a| super::play(&state, &source, PlayStyle::Play, Some(1), a),
		super::poise::playnext,
		super::playnext_help(),
		query.map(|q| q.0).as_ref(),
	)
	.await
}

pub async fn playnow(
	State(state): State<BotState>,
	jar: CookieJar,
	query: Option<Query<PlayArgs>>,
) -> Html<String> {
	let source = match extract_source(&jar, state.data.read().await.get_expect::<AeadKey>()) {
		Err(e) => return Html(response_to_html_string(Err(e))),
		Ok(source) => source,
	};

	run(
		|a| super::play(&state, &source, PlayStyle::Play, Some(0), a),
		super::poise::playnow,
		super::playnow_help(),
		query.map(|q| q.0).as_ref(),
	)
	.await
}
