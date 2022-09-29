use std::num::ParseIntError;

use crate::util::{CommandResult, Context, Respond};

enum ParseCodeError {
	ParseIntError(ParseIntError),
	InvalidCode(u32),
}

fn parse_code(string: &str) -> Result<char, ParseCodeError> {
	use ParseCodeError::*;

	let code = if string.starts_with("0x") {
		u32::from_str_radix(&string[2..], 16)
	} else {
		u32::from_str_radix(string, 10)
	};

	match code {
		Err(parse_error) => Err(ParseIntError(parse_error)),
		Ok(c) => match std::char::from_u32(c) {
			None => Err(InvalidCode(c)),
			Some(c) => Ok(c),
		},
	}
}

/// Print the characters based on the unicode code point.
///
/// The code point can be specified in either decimal or hexidecimal (by preceding it with 0x).
///
/// **Usage:** `unicode [codepoint...]`
///
/// **Examples:**
/// - `unicode 0x252C 0x2500 0x252C 0x30CE 0x28 0x20 0xBA 0x20 0x5F 0x20 0xBA 0x30CE 0x29`
#[poise::command(category = "unicode", prefix_command, slash_command)]
pub async fn unicode(
	ctx: Context<'_>,
	#[description = "Unicode code points"] codepoints: Vec<String>,
) -> CommandResult {
	let mut chars = Vec::with_capacity(codepoints.len());
	let mut reply = None;

	for code_str in &codepoints {
		let c = match parse_code(&code_str) {
			Err(_) => {
				reply = Some(format!("Invalid character code: {}", code_str));
				break;
			}
			Ok(c) => c,
		};

		chars.push(c);
	}

	if reply.is_none() {
		reply = Some(chars.into_iter().collect());
	}

	ctx.respond_ok(&reply.unwrap().into()).await?;

	Ok(())
}
