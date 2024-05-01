use axum::extract::{Query, State};
use axum::response::Html;

use axum_extra::extract::CookieJar;

use crate::commands::http::{extract_source, render_response};
use crate::commands::BotState;
use crate::util::GetExpect;
use crate::AeadKey;

use super::{LoopArgs, MoveArgs, QueueArgs, SkipArgs};

pub async fn stop(State(state): State<BotState>, jar: CookieJar) -> Html<String> {
	let source = match extract_source(&jar, state.data.read().await.get_expect::<AeadKey>()) {
		Err(e) => return render_response(Err(e)),
		Ok(source) => source,
	};

	render_response(super::stop(&state, &source).await)
}

pub async fn skip(
	State(state): State<BotState>,
	jar: CookieJar,
	Query(args): Query<SkipArgs>,
) -> Html<String> {
	let source = match extract_source(&jar, state.data.read().await.get_expect::<AeadKey>()) {
		Err(e) => return render_response(Err(e)),
		Ok(source) => source,
	};

	render_response(super::skip(&state, &source, &args).await)
}

pub async fn pause(State(state): State<BotState>, jar: CookieJar) -> Html<String> {
	let source = match extract_source(&jar, state.data.read().await.get_expect::<AeadKey>()) {
		Err(e) => return render_response(Err(e)),
		Ok(source) => source,
	};

	render_response(super::pause(&state, &source).await)
}

pub async fn unpause(State(state): State<BotState>, jar: CookieJar) -> Html<String> {
	let source = match extract_source(&jar, state.data.read().await.get_expect::<AeadKey>()) {
		Err(e) => return render_response(Err(e)),
		Ok(source) => source,
	};

	render_response(super::unpause(&state, &source).await)
}

pub async fn queue(
	State(state): State<BotState>,
	jar: CookieJar,
	Query(args): Query<QueueArgs>,
) -> Html<String> {
	let source = match extract_source(&jar, state.data.read().await.get_expect::<AeadKey>()) {
		Err(e) => return render_response(Err(e)),
		Ok(source) => source,
	};

	render_response(super::queue(&state, &source, args).await)
}

pub async fn shuffle(State(state): State<BotState>, jar: CookieJar) -> Html<String> {
	let source = match extract_source(&jar, state.data.read().await.get_expect::<AeadKey>()) {
		Err(e) => return render_response(Err(e)),
		Ok(source) => source,
	};

	render_response(super::shuffle(&state, &source, 1).await)
}

pub async fn shufflenow(State(state): State<BotState>, jar: CookieJar) -> Html<String> {
	let source = match extract_source(&jar, state.data.read().await.get_expect::<AeadKey>()) {
		Err(e) => return render_response(Err(e)),
		Ok(source) => source,
	};

	render_response(super::shuffle(&state, &source, 0).await)
}

pub async fn r#loop(
	State(state): State<BotState>,
	jar: CookieJar,
	Query(args): Query<LoopArgs>,
) -> Html<String> {
	let source = match extract_source(&jar, state.data.read().await.get_expect::<AeadKey>()) {
		Err(e) => return render_response(Err(e)),
		Ok(source) => source,
	};

	render_response(super::r#loop(&state, &source, &args).await)
}

pub async fn r#move(
	State(state): State<BotState>,
	jar: CookieJar,
	Query(args): Query<MoveArgs>,
) -> Html<String> {
	let source = match extract_source(&jar, state.data.read().await.get_expect::<AeadKey>()) {
		Err(e) => return render_response(Err(e)),
		Ok(source) => source,
	};

	render_response(super::r#move(&state, &source, args).await)
}
