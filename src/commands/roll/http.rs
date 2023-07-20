use axum::extract::Query;
use axum::response::Html;

use crate::commands::http::render_response;

use super::RollArgs;

pub async fn roll(Query(args): Query<RollArgs>) -> Html<String> {
	render_response(super::roll(&args).await)
}
