use axum::extract::Query;
use axum::response::Html;

use crate::commands::http::run;

use super::{CmdArgs, CmdlistArgs};

pub async fn cmd(Query(args): Query<CmdArgs>) -> Html<String> {
	run(super::cmd, &args).await
}

pub async fn cmdlist(Query(args): Query<CmdlistArgs>) -> Html<String> {
	run(super::cmdlist, &args).await
}
