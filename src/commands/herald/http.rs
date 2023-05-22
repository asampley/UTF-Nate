use axum::extract::{Query, State};
use axum::response::Html;
use axum_extra::extract::cookie::CookieJar;

use crate::commands::http::{extract_source, response_to_html, run};
use crate::commands::BotState;

use super::{IntroBotArgs, IntroOutroArgs, IntroOutroMode};

pub async fn intro(
	State(state): State<BotState>,
	jar: CookieJar,
	query: Option<Query<IntroOutroArgs>>,
) -> Html<String> {
	let source = match extract_source(&jar) {
		Err(e) => return Html(response_to_html(Err(e)).to_string()),
		Ok(source) => source,
	};

	run(
		|a| super::intro_outro(&state, &source, IntroOutroMode::Intro, a),
		super::poise::intro,
		super::intro_help(),
		query.map(|q| q.0).as_ref(),
	)
	.await
}

pub async fn introbot(
	State(state): State<BotState>,
	jar: CookieJar,
	query: Option<Query<IntroBotArgs>>,
) -> Html<String> {
	let source = match extract_source(&jar) {
		Err(e) => return Html(response_to_html(Err(e)).to_string()),
		Ok(source) => source,
	};

	run(
		|a| super::introbot(&state, &source, a),
		super::poise::introbot,
		super::introbot_help(),
		query.map(|q| q.0).as_ref(),
	)
	.await
}

pub async fn outro(
	jar: CookieJar,
	State(state): State<BotState>,
	query: Option<Query<IntroOutroArgs>>,
) -> Html<String> {
	let source = match extract_source(&jar) {
		Err(e) => return Html(response_to_html(Err(e)).to_string()),
		Ok(source) => source,
	};

	run(
		|a| super::intro_outro(&state, &source, IntroOutroMode::Outro, a),
		super::poise::outro,
		super::outro_help(),
		query.map(|q| q.0).as_ref(),
	)
	.await
}
