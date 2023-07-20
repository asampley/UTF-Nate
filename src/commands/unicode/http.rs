use axum::extract::Query;
use axum::response::Html;

use crate::commands::http::render_response;

use super::UnicodeArgs;

pub async fn unicode(Query(args): Query<UnicodeArgs>) -> Html<String> {
	render_response(super::unicode(&args).await)
}
