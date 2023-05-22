use axum::extract::Query;
use axum::response::Html;

use crate::commands::http::run;

use super::RollArgs;

pub async fn roll(query: Option<Query<RollArgs>>) -> Html<String> {
	run(
		super::roll,
		super::poise::roll,
		super::roll_help(),
		query.map(|q| q.0).as_ref(),
	)
	.await
}
