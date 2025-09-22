use axum::extract::State;
use axum::response::Html;

use axum_extra::extract::CookieJar;

use crate::AeadKey;
use crate::commands::BotState;
use crate::commands::http::{extract_source, render_response};
use crate::util::GetExpect;

pub async fn summon(State(state): State<BotState>, jar: CookieJar) -> Html<String> {
	let source = match extract_source(&jar, state.data.read().await.get_expect::<AeadKey>()) {
		Err(e) => return render_response(Err(e)),
		Ok(source) => source,
	};

	render_response(super::summon(&state, &source).await)
}

pub async fn banish(State(state): State<BotState>, jar: CookieJar) -> Html<String> {
	let source = match extract_source(&jar, state.data.read().await.get_expect::<AeadKey>()) {
		Err(e) => return render_response(Err(e)),
		Ok(source) => source,
	};

	render_response(super::banish(&state, &source).await)
}
