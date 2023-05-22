use axum::extract::{Query, State};
use axum::response::Html;

use axum_extra::extract::CookieJar;

use crate::commands::http::{extract_source, response_to_html, run};
use crate::commands::BotState;
use crate::util::GetExpect;
use crate::AeadKey;

use super::{LoopArgs, SkipArgs};

pub async fn stop(State(state): State<BotState>, jar: CookieJar) -> Html<String> {
	let source = match extract_source(&jar, state.data.read().await.get_expect::<AeadKey>()) {
		Err(e) => return Html(response_to_html(Err(e)).to_string()),
		Ok(source) => source,
	};

	run(
		|_| super::stop(&state, &source),
		super::poise::stop,
		super::stop_help(),
		Some(()),
	)
	.await
}

pub async fn skip(
	State(state): State<BotState>,
	jar: CookieJar,
	query: Option<Query<SkipArgs>>,
) -> Html<String> {
	let source = match extract_source(&jar, state.data.read().await.get_expect::<AeadKey>()) {
		Err(e) => return Html(response_to_html(Err(e)).to_string()),
		Ok(source) => source,
	};

	run(
		|a| super::skip(&state, &source, a),
		super::poise::skip,
		super::skip_help(),
		query.map(|q| q.0).as_ref(),
	)
	.await
}

pub async fn pause(State(state): State<BotState>, jar: CookieJar) -> Html<String> {
	let source = match extract_source(&jar, state.data.read().await.get_expect::<AeadKey>()) {
		Err(e) => return Html(response_to_html(Err(e)).to_string()),
		Ok(source) => source,
	};

	run(
		|_| super::pause(&state, &source),
		super::poise::pause,
		super::pause_help(),
		Some(()),
	)
	.await
}

pub async fn unpause(State(state): State<BotState>, jar: CookieJar) -> Html<String> {
	let source = match extract_source(&jar, state.data.read().await.get_expect::<AeadKey>()) {
		Err(e) => return Html(response_to_html(Err(e)).to_string()),
		Ok(source) => source,
	};

	run(
		|_| super::unpause(&state, &source),
		super::poise::unpause,
		super::unpause_help(),
		Some(()),
	)
	.await
}

pub async fn queue(State(state): State<BotState>, jar: CookieJar) -> Html<String> {
	let source = match extract_source(&jar, state.data.read().await.get_expect::<AeadKey>()) {
		Err(e) => return Html(response_to_html(Err(e)).to_string()),
		Ok(source) => source,
	};

	run(
		|_| super::queue(&state, &source),
		super::poise::queue,
		super::queue_help(),
		Some(()),
	)
	.await
}

pub async fn shuffle(State(state): State<BotState>, jar: CookieJar) -> Html<String> {
	let source = match extract_source(&jar, state.data.read().await.get_expect::<AeadKey>()) {
		Err(e) => return Html(response_to_html(Err(e)).to_string()),
		Ok(source) => source,
	};

	run(
		|_| super::shuffle(&state, &source, 1),
		super::poise::shuffle,
		super::shuffle_help(),
		Some(()),
	)
	.await
}

pub async fn shufflenow(State(state): State<BotState>, jar: CookieJar) -> Html<String> {
	let source = match extract_source(&jar, state.data.read().await.get_expect::<AeadKey>()) {
		Err(e) => return Html(response_to_html(Err(e)).to_string()),
		Ok(source) => source,
	};

	run(
		|_| super::shuffle(&state, &source, 0),
		super::poise::shufflenow,
		super::shufflenow_help(),
		Some(()),
	)
	.await
}

pub async fn r#loop(
	State(state): State<BotState>,
	jar: CookieJar,
	query: Option<Query<LoopArgs>>,
) -> Html<String> {
	let source = match extract_source(&jar, state.data.read().await.get_expect::<AeadKey>()) {
		Err(e) => return Html(response_to_html(Err(e)).to_string()),
		Ok(source) => source,
	};

	run(
		|a| super::r#loop(&state, &source, a),
		super::poise::r#loop,
		super::loop_help(),
		query.map(|q| q.0).as_ref(),
	)
	.await
}
