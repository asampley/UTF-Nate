use axum::extract::{Query, State};
use axum::response::Html;
use axum_extra::extract::cookie::CookieJar;

use crate::commands::http::{extract_source, render_response};
use crate::commands::BotState;
use crate::util::GetExpect;
use crate::AeadKey;

use super::{IntroBotArgs, IntroOutroArgs, IntroOutroMode};

pub async fn intro(
	State(state): State<BotState>,
	jar: CookieJar,
	Query(args): Query<IntroOutroArgs>,
) -> Html<String> {
	let source = match extract_source(&jar, state.data.read().await.get_expect::<AeadKey>()) {
		Err(e) => return render_response(Err(e)),
		Ok(source) => source,
	};

	render_response(super::intro_outro(&state, &source, IntroOutroMode::Intro, &args).await)
}

pub async fn introbot(
	State(state): State<BotState>,
	jar: CookieJar,
	Query(args): Query<IntroBotArgs>,
) -> Html<String> {
	let source = match extract_source(&jar, state.data.read().await.get_expect::<AeadKey>()) {
		Err(e) => return render_response(Err(e)),
		Ok(source) => source,
	};

	render_response(super::introbot(&state, &source, &args).await)
}

pub async fn outro(
	jar: CookieJar,
	State(state): State<BotState>,
	Query(args): Query<IntroOutroArgs>,
) -> Html<String> {
	let source = match extract_source(&jar, state.data.read().await.get_expect::<AeadKey>()) {
		Err(e) => return render_response(Err(e)),
		Ok(source) => source,
	};

	render_response(super::intro_outro(&state, &source, IntroOutroMode::Outro, &args).await)
}
