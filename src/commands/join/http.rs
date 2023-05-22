use axum::extract::State;
use axum::response::Html;

use axum_extra::extract::CookieJar;

use crate::commands::http::{extract_source, response_to_html, run};
use crate::commands::BotState;
use crate::util::GetExpect;
use crate::AeadKey;

pub async fn summon(State(state): State<BotState>, jar: CookieJar) -> Html<String> {
	let source = match extract_source(&jar, state.data.read().await.get_expect::<AeadKey>()) {
		Err(e) => return Html(response_to_html(Err(e)).to_string()),
		Ok(source) => source,
	};

	run(
		|_| super::summon(&state, &source),
		super::poise::summon,
		super::summon_help(),
		Some(()),
	)
	.await
}

pub async fn banish(State(state): State<BotState>, jar: CookieJar) -> Html<String> {
	let source = match extract_source(&jar, state.data.read().await.get_expect::<AeadKey>()) {
		Err(e) => return Html(response_to_html(Err(e)).to_string()),
		Ok(source) => source,
	};

	run(
		|_| super::banish(&state, &source),
		super::poise::banish,
		super::banish_help(),
		Some(()),
	)
	.await
}
