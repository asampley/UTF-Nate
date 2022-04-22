use itertools::Itertools;

use log::{error, info};

use std::fs::read_dir;
use std::path::Path;
use std::process;
use std::process::Stdio;

use crate::commands::external::cmd_path;
use crate::util::*;

pub async fn cmd(
	command: Option<&str>,
	args: impl Iterator<Item = &str>,
) -> Result<Response, Response> {
	let command = match command {
		Some(command) => cmd_path().join(&command),
		None => return Err("Must provide a command".into()),
	};

	if !sandboxed_exists(&cmd_path(), &command) {
		return Err("Invalid command".into());
	}

	let output = process::Command::new(&command)
		.args(args)
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

pub async fn cmdlist(path: Option<&str>) -> Result<Response, Response> {
	let dir = cmd_path().join(Path::new(match path {
		None => "",
		Some(ref path) => path,
	}));

	let dir = dir.canonicalize().map_err(|_| "Invalid directory")?;

	if !sandboxed_exists(&cmd_path(), &dir) {
		return Err("Invalid directory".into());
	}

	match read_dir(dir) {
		Err(reason) => {
			error!("Unable to read directory: {:?}", reason);
			return Err("Invalid directory".into());
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

			return Ok(("```\n".to_owned() + &message + "\n```").into());
		}
	}
}
