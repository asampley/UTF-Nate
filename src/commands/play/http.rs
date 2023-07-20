use axum::extract::{Query, State};
use axum::response::Html;

use axum_extra::extract::CookieJar;

use crate::commands::http::{extract_source, render_response};
use crate::commands::BotState;
use crate::util::GetExpect;
use crate::AeadKey;

use super::{PlayArgs, PlayStyle};

pub async fn clip(
	State(state): State<BotState>,
	jar: CookieJar,
	Query(args): Query<PlayArgs>,
) -> Html<String> {
	let source = match extract_source(&jar, state.data.read().await.get_expect::<AeadKey>()) {
		Err(e) => return render_response(Err(e)),
		Ok(source) => source,
	};

	render_response(super::play(&state, &source, PlayStyle::Clip, None, &args).await)
}

pub async fn play(
	State(state): State<BotState>,
	jar: CookieJar,
	Query(args): Query<PlayArgs>,
) -> Html<String> {
	let source = match extract_source(&jar, state.data.read().await.get_expect::<AeadKey>()) {
		Err(e) => return render_response(Err(e)),
		Ok(source) => source,
	};

	render_response(super::play(&state, &source, PlayStyle::Play, None, &args).await)
}

pub async fn playnext(
	State(state): State<BotState>,
	jar: CookieJar,
	Query(args): Query<PlayArgs>,
) -> Html<String> {
	let source = match extract_source(&jar, state.data.read().await.get_expect::<AeadKey>()) {
		Err(e) => return render_response(Err(e)),
		Ok(source) => source,
	};

	render_response(super::play(&state, &source, PlayStyle::Play, Some(1), &args).await)
}

pub async fn playnow(
	State(state): State<BotState>,
	jar: CookieJar,
	Query(args): Query<PlayArgs>,
) -> Html<String> {
	let source = match extract_source(&jar, state.data.read().await.get_expect::<AeadKey>()) {
		Err(e) => return render_response(Err(e)),
		Ok(source) => source,
	};

	render_response(super::play(&state, &source, PlayStyle::Play, Some(0), &args).await)
}
