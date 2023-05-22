use axum::extract::Query;
use axum::response::Html;

use crate::commands::http::run;

use super::{CmdArgs, CmdlistArgs};

pub async fn cmd(query: Option<Query<CmdArgs>>) -> Html<String> {
	run(
		super::cmd,
		super::poise::cmd,
		super::cmd_help(),
		query.map(|q| q.0).as_ref(),
	)
	.await
}

pub async fn cmdlist(query: Option<Query<CmdlistArgs>>) -> Html<String> {
	run(
		super::cmdlist,
		super::poise::cmdlist,
		super::cmdlist_help(),
		query.map(|q| q.0).as_ref(),
	)
	.await
}
