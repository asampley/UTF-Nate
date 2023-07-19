use axum::extract::Query;
use axum::response::Html;

use crate::commands::http::run;

use super::RollArgs;

pub async fn roll(Query(args): Query<RollArgs>) -> Html<String> {
	run(super::roll, &args).await
}
