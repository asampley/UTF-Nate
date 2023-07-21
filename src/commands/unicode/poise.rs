use crate::commands::{run, CustomData};
use crate::util::{CommandResult, Context};

use super::UnicodeArgs;

#[poise::command(
	category = "unicode",
	prefix_command,
	slash_command,
	custom_data = "CustomData::new(super::unicode_help)"
)]
pub async fn unicode(
	ctx: Context<'_>,
	#[description = "Unicode code points"] codepoints: String,
) -> CommandResult {
	run(&ctx, super::unicode(&UnicodeArgs { codepoints })).await
}
