use itertools::Itertools;

use once_cell::sync::Lazy;

use serde::{Deserialize, Serialize};

use tracing::{error, info};

use std::fs::read_dir;
use std::path::{Path, PathBuf};
use std::process;
use std::process::Stdio;

use crate::util::*;
use crate::RESOURCE_PATH;

#[cfg(feature = "http-interface")]
pub mod http;
pub mod poise;

pub static CMD_PATH: Lazy<PathBuf> = Lazy::new(|| RESOURCE_PATH.join("cmd/"));

pub const fn cmd_help() -> &'static str {
	include_str!("cmd.md")
}

pub const fn cmdlist_help() -> &'static str {
	include_str!("cmdlist.md")
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CmdArgs {
	command: String,
	args: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CmdlistArgs {
	path: Option<String>,
}

#[tracing::instrument(level = "info", ret)]
pub async fn cmd(CmdArgs { command, args }: &CmdArgs) -> Result<Response, Response> {
	let command = CMD_PATH.join(command);

	if !sandboxed_exists(&CMD_PATH, &command) {
		return Err("Invalid command".into());
	}

	let output = process::Command::new(&command)
		.args(serenity::utils::parse_quotes(args))
		.stdin(Stdio::null())
		.output();

	match output {
		Ok(output) => {
			let stdout = String::from_utf8_lossy(&output.stdout);
			let stderr = String::from_utf8_lossy(&output.stderr);

			info!("Stdout of command: {}", stdout);
			info!("Stderr of command: {}", stderr);

			if output.status.success() {
				Ok(stdout.as_ref().into())
			} else {
				Err("Error executing command. Please check logs".into())
			}
		}
		Err(reason) => {
			error!("Error executing command {:?}: {:?}", command, reason);

			Err("Error executing command".into())
		}
	}
}

#[tracing::instrument(level = "info", ret)]
pub async fn cmdlist(CmdlistArgs { path }: &CmdlistArgs) -> Result<Response, Response> {
	let dir = CMD_PATH.join(Path::new(match path {
		None => "",
		Some(path) => path,
	}));

	let dir = dir.canonicalize().map_err(|_| "Invalid directory")?;

	if !sandboxed_exists(&CMD_PATH, &dir) {
		return Err("Invalid directory".into());
	}

	match read_dir(dir) {
		Err(reason) => {
			error!("Unable to read directory: {:?}", reason);
			Err("Invalid directory".into())
		}
		Ok(dir_iter) => {
			let message = dir_iter
				.filter_map(|e| e.ok())
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
	}
}
