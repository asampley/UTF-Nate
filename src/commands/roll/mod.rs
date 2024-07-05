use caith::Roller;

use serde::{Deserialize, Serialize};

use crate::util::Response;

#[cfg(feature = "http-interface")]
pub mod http;
pub mod poise;

#[derive(Debug, Serialize, Deserialize)]
pub struct RollArgs {
	pub expression: String,
}

#[doc = include_str!("help/roll.md")]
#[tracing::instrument(level = "info", ret)]
pub async fn roll(RollArgs { expression }: &RollArgs) -> Result<Response, Response> {
	Ok(Roller::new(expression)
		.and_then(|r| r.roll())
		.map_err(|e| e.to_string())?
		.to_string()
		.into())
}

pub const fn roll_help() -> &'static str {
	include_str!("help/roll.md")
}
