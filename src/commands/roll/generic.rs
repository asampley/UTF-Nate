use caith::Roller;

use crate::util::Response;

pub async fn roll(expression: String) -> Result<Response, Response> {
	Ok(Roller::new(&expression)
		.and_then(|r| r.roll())
		.map_err(|e| e.to_string())?
		.to_string()
		.into())
}
