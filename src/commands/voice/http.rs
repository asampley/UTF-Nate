use axum::extract::{Query, State};
use axum::response::Html;

use axum_extra::extract::CookieJar;

use serde::{Deserialize, Serialize};

use crate::AeadKey;
use crate::audio::PlayStyle;
use crate::commands::BotState;
use crate::commands::http::{extract_source, render_response};
use crate::commands::voice::ListArgs;
use crate::util::{GetExpect, from_str_blank_as_none};

use super::VolumeMode;

#[derive(Debug, Deserialize, Serialize)]
pub struct VolumeSetArgs {
	#[serde(deserialize_with = "from_str_blank_as_none")]
	volume: Option<f32>,
}

pub async fn volume(state: State<BotState>, jar: CookieJar) -> Html<String> {
	volume_get(state, jar).await
}

pub async fn volume_get(State(state): State<BotState>, jar: CookieJar) -> Html<String> {
	let source = match extract_source(&jar, state.data.read().await.get_expect::<AeadKey>()) {
		Err(e) => return render_response(Err(e)),
		Ok(source) => source,
	};

	render_response(super::volume(&state, &source, VolumeMode::ConfigAllStyles).await)
}

pub async fn volume_play(
	State(state): State<BotState>,
	jar: CookieJar,
	Query(args): Query<VolumeSetArgs>,
) -> Html<String> {
	let source = match extract_source(&jar, state.data.read().await.get_expect::<AeadKey>()) {
		Err(e) => return render_response(Err(e)),
		Ok(source) => source,
	};

	render_response(
		super::volume(
			&state,
			&source,
			VolumeMode::Config(PlayStyle::Play, args.volume),
		)
		.await,
	)
}

pub async fn volume_clip(
	State(state): State<BotState>,
	jar: CookieJar,
	Query(args): Query<VolumeSetArgs>,
) -> Html<String> {
	let source = match extract_source(&jar, state.data.read().await.get_expect::<AeadKey>()) {
		Err(e) => return render_response(Err(e)),
		Ok(source) => source,
	};

	render_response(
		super::volume(
			&state,
			&source,
			VolumeMode::Config(PlayStyle::Clip, args.volume),
		)
		.await,
	)
}

pub async fn volume_now(
	State(state): State<BotState>,
	jar: CookieJar,
	Query(args): Query<VolumeSetArgs>,
) -> Html<String> {
	let source = match extract_source(&jar, state.data.read().await.get_expect::<AeadKey>()) {
		Err(e) => return render_response(Err(e)),
		Ok(source) => source,
	};

	render_response(super::volume(&state, &source, VolumeMode::Current(args.volume)).await)
}

pub async fn list(Query(args): Query<ListArgs>) -> Html<String> {
	render_response(super::list(args).await)
}
