use axum::{extract::Query, response::Html};

use crate::commands::http::render_response;

use super::HelpArgs;

pub async fn help(Query(args): Query<HelpArgs>) -> Html<String> {
	render_response(super::help(&args).await)
}
