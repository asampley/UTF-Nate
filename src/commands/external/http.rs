use axum::extract::Query;
use axum::response::Html;

use crate::commands::http::render_response;

use super::{CmdArgs, CmdlistArgs};

pub async fn cmd(Query(args): Query<CmdArgs>) -> Html<String> {
	render_response(super::cmd(args).await)
}

pub async fn cmdlist(Query(args): Query<CmdlistArgs>) -> Html<String> {
	render_response(super::cmdlist(&args).await)
}
