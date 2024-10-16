use serde::{Deserialize, Serialize};

use thiserror::Error;

use std::num::ParseIntError;

use crate::util::Response;

#[cfg(feature = "http-interface")]
pub mod http;
pub mod poise;

#[derive(Debug, Deserialize, Serialize)]
pub struct UnicodeArgs {
	codepoints: String,
}

#[derive(Debug, Error)]
enum ParseCodeError {
	#[error("unable to parse int")]
	ParseIntError(#[from] ParseIntError),

	#[error("invalid unicode code")]
	InvalidCode(u32),
}

fn parse_code(string: &str) -> Result<char, ParseCodeError> {
	use ParseCodeError::*;

	let code = match string.strip_prefix("0x") {
		Some(code) => u32::from_str_radix(code, 16),
		None => string.parse(),
	}?;

	match std::char::from_u32(code) {
		None => Err(InvalidCode(code)),
		Some(c) => Ok(c),
	}
}

#[tracing::instrument(level = "info", ret)]
pub async fn unicode(args: &UnicodeArgs) -> Result<Response, Response> {
	let codepoints = serenity::utils::parse_quotes(&args.codepoints);

	let mut chars = Vec::with_capacity(codepoints.len());

	for code_str in &codepoints {
		match parse_code(code_str) {
			Err(_) => {
				return Err(format!("Invalid character code: {}", code_str).into());
			}
			Ok(c) => chars.push(c),
		};
	}

	Ok(chars.into_iter().collect::<String>().into())
}

pub const fn unicode_help() -> &'static str {
	include_str!("help/unicode.md")
}

#[cfg(test)]
mod test {
	use super::*;

	use itertools::Itertools;

	#[tokio::test]
	async fn unicode_single_character() {
		let codepoints = [41, 123, 0x7fff];

		for c in codepoints {
			assert_eq!(
				char::from_u32(c).into_iter().collect::<String>(),
				unicode(&UnicodeArgs {
					codepoints: format!("0x{c:x}")
				})
				.await
				.map(|response| response.text)
				.unwrap()
			);
		}
	}

	#[tokio::test]
	async fn unicode_multiple_characters() {
		let codepoints = [41, 123, 0x7fff];

		for combo in (0..codepoints.len()).flat_map(|k| codepoints.into_iter().combinations(k)) {
			assert_eq!(
				combo
					.iter()
					.flat_map(|&c| char::from_u32(c))
					.collect::<String>(),
				unicode(&UnicodeArgs {
					codepoints: combo.iter().map(|c| format!("0x{c:x} ")).join(" ")
				})
				.await
				.map(|response| response.text)
				.unwrap()
			);
		}
	}
}
