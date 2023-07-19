use axum::extract::{Query, State};
use axum::response::Html;

use axum_extra::extract::CookieJar;

use crate::commands::http::{extract_source, response_to_html_string, run};
use crate::commands::BotState;
use crate::util::GetExpect;
use crate::AeadKey;

use super::{LoopArgs, SkipArgs};

pub async fn stop(State(state): State<BotState>, jar: CookieJar) -> Html<String> {
	let source = match extract_source(&jar, state.data.read().await.get_expect::<AeadKey>()) {
		Err(e) => return Html(response_to_html_string(Err(e))),
		Ok(source) => source,
	};

	run(|_| super::stop(&state, &source), ()).await
}

pub async fn skip(
	State(state): State<BotState>,
	jar: CookieJar,
	Query(args): Query<SkipArgs>,
) -> Html<String> {
	let source = match extract_source(&jar, state.data.read().await.get_expect::<AeadKey>()) {
		Err(e) => return Html(response_to_html_string(Err(e))),
		Ok(source) => source,
	};

	run(|a| super::skip(&state, &source, a), &args).await
}

pub async fn pause(State(state): State<BotState>, jar: CookieJar) -> Html<String> {
	let source = match extract_source(&jar, state.data.read().await.get_expect::<AeadKey>()) {
		Err(e) => return Html(response_to_html_string(Err(e))),
		Ok(source) => source,
	};

	run(|_| super::pause(&state, &source), ()).await
}

pub async fn unpause(State(state): State<BotState>, jar: CookieJar) -> Html<String> {
	let source = match extract_source(&jar, state.data.read().await.get_expect::<AeadKey>()) {
		Err(e) => return Html(response_to_html_string(Err(e))),
		Ok(source) => source,
	};

	run(|_| super::unpause(&state, &source), ()).await
}

pub async fn queue(State(state): State<BotState>, jar: CookieJar) -> Html<String> {
	let source = match extract_source(&jar, state.data.read().await.get_expect::<AeadKey>()) {
		Err(e) => return Html(response_to_html_string(Err(e))),
		Ok(source) => source,
	};

	run(|_| super::queue(&state, &source), ()).await
}

pub async fn shuffle(State(state): State<BotState>, jar: CookieJar) -> Html<String> {
	let source = match extract_source(&jar, state.data.read().await.get_expect::<AeadKey>()) {
		Err(e) => return Html(response_to_html_string(Err(e))),
		Ok(source) => source,
	};

	run(|_| super::shuffle(&state, &source, 1), ()).await
}

pub async fn shufflenow(State(state): State<BotState>, jar: CookieJar) -> Html<String> {
	let source = match extract_source(&jar, state.data.read().await.get_expect::<AeadKey>()) {
		Err(e) => return Html(response_to_html_string(Err(e))),
		Ok(source) => source,
	};

	run(|_| super::shuffle(&state, &source, 0), ()).await
}

pub async fn r#loop(
	State(state): State<BotState>,
	jar: CookieJar,
	Query(args): Query<LoopArgs>,
) -> Html<String> {
	let source = match extract_source(&jar, state.data.read().await.get_expect::<AeadKey>()) {
		Err(e) => return Html(response_to_html_string(Err(e))),
		Ok(source) => source,
	};

	run(|a| super::r#loop(&state, &source, a), &args).await
}
