use axum::extract::State;
use axum::response::Html;

use axum_extra::extract::CookieJar;

use crate::commands::http::{extract_source, response_to_html_string, run};
use crate::commands::BotState;
use crate::util::GetExpect;
use crate::AeadKey;

pub async fn summon(State(state): State<BotState>, jar: CookieJar) -> Html<String> {
	let source = match extract_source(&jar, state.data.read().await.get_expect::<AeadKey>()) {
		Err(e) => return Html(response_to_html_string(Err(e))),
		Ok(source) => source,
	};

	run(|_| super::summon(&state, &source), ()).await
}

pub async fn banish(State(state): State<BotState>, jar: CookieJar) -> Html<String> {
	let source = match extract_source(&jar, state.data.read().await.get_expect::<AeadKey>()) {
		Err(e) => return Html(response_to_html_string(Err(e))),
		Ok(source) => source,
	};

	run(|_| super::banish(&state, &source), ()).await
}
