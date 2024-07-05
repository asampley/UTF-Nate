use itertools::Itertools;

use once_cell::sync::Lazy;

use serde::{Deserialize, Serialize};

use tap::TapFallible;
use tracing::{error, info};

use std::fs::read_dir;
use std::path::PathBuf;
use std::process;
use std::process::Stdio;

use crate::util::*;
use crate::RESOURCE_PATH;

#[cfg(feature = "http-interface")]
pub mod http;
pub mod poise;

pub static CMD_PATH: Lazy<PathBuf> = Lazy::new(|| RESOURCE_PATH.join("cmd/"));

pub const fn cmd_help() -> &'static str {
	include_str!("help/cmd.md")
}

pub const fn cmdlist_help() -> &'static str {
	include_str!("help/cmdlist.md")
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CmdArgs {
	command: String,
	args: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CmdlistArgs {
	path: Option<String>,
}

#[tracing::instrument(level = "info", ret)]
pub async fn cmd(CmdArgs { command, args }: CmdArgs) -> Result<Response, Response> {
	let command = sandboxed_join(&CMD_PATH, command).ok_or("Invalid command")?;

	tokio::task::spawn_blocking(move || {
		let output = process::Command::new(&command)
			.args(args.iter().flat_map(serenity::utils::parse_quotes))
			.stdin(Stdio::null())
			.output()
			.tap_err(|reason| error!("Error executing command {:?}: {:?}", command, reason))
			.map_err(|_| "Error executing command")?;

		let stdout = String::from_utf8_lossy(&output.stdout);
		let stderr = String::from_utf8_lossy(&output.stderr);

		info!("Stdout of command: {}", stdout);
		info!("Stderr of command: {}", stderr);

		if output.status.success() {
			Ok(stdout.as_ref().into())
		} else {
			Err("Error executing command. Please check logs".into())
		}
	})
	.await
	.tap_err(|e| error!("Failed to join blocking task: {e:?}"))
	.unwrap_or_else(|_| Err("Error executing command".into()))
}

#[tracing::instrument(level = "info", ret)]
pub async fn cmdlist(CmdlistArgs { path }: &CmdlistArgs) -> Result<Response, Response> {
	let dir =
		sandboxed_join(&CMD_PATH, path.as_deref().unwrap_or("")).ok_or("Invalid directory")?;

	let dir_iter = read_dir(dir)
		.tap_err(|reason| error!("Unable to read directory: {:?}", reason))
		.map_err(|_| "Invalid directory")?;

	let message = dir_iter
		.filter_map(|e| e.tap_err(|e| error!("{:?}", e)).ok())
		.map(|e| {
			(
				e.path()
					.file_stem()
					.and_then(|f| f.to_str())
					.map(|f| f.to_owned()),
				e.file_type(),
			)
		})
		.filter(|(f, t)| f.is_some() && t.is_ok())
		.map(|(f, t)| (f.unwrap(), t.unwrap()))
		.sorted_by(|(f0, t0), (f1, t1)| {
			(!t0.is_dir(), f0.to_lowercase()).cmp(&(!t1.is_dir(), f1.to_lowercase()))
		})
		.map(|(f, t)| format!("{: <20}", f + if t.is_dir() { "/" } else { "" }))
		.chunks(3)
		.into_iter()
		.map(|chunk| chunk.fold("".to_owned(), |acc, s| acc + &s))
		.fold("".to_owned(), |acc, s| acc + "\n" + &s);

	Ok(("```\n".to_owned() + &message + "\n```").into())
}
