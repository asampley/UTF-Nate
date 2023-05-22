use axum::extract::Query;
use axum::response::Html;

use crate::commands::http::run;

use super::UnicodeArgs;

pub async fn unicode(query: Option<Query<UnicodeArgs>>) -> Html<String> {
	run(
		super::unicode,
		super::poise::unicode,
		super::unicode_help(),
		query.map(|q| q.0).as_ref(),
	)
	.await
}
