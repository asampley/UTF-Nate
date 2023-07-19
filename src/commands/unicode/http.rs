use axum::extract::Query;
use axum::response::Html;

use crate::commands::http::run;

use super::UnicodeArgs;

pub async fn unicode(Query(args): Query<UnicodeArgs>) -> Html<String> {
	run(super::unicode, &args).await
}
