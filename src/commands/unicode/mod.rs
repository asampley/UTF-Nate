use serde::{Deserialize, Serialize};

use std::num::ParseIntError;

use crate::util::Response;

#[cfg(feature = "http-interface")]
pub mod http;
pub mod poise;

#[derive(Debug, Deserialize, Serialize)]
pub struct UnicodeArgs {
	codepoints: String,
}

enum ParseCodeError {
	ParseIntError(ParseIntError),
	InvalidCode(u32),
}

fn parse_code(string: &str) -> Result<char, ParseCodeError> {
	use ParseCodeError::*;

	let code = if let Some(code) = string.strip_prefix("0x") {
		u32::from_str_radix(code, 16)
	} else {
		string.parse()
	};

	match code {
		Err(parse_error) => Err(ParseIntError(parse_error)),
		Ok(c) => match std::char::from_u32(c) {
			None => Err(InvalidCode(c)),
			Some(c) => Ok(c),
		},
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
	include_str!("unicode.md")
}
