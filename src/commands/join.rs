use crate::commands::run;
use crate::util::{CommandResult, Context};

mod generic;

/// Summon the bot to the voice channel the user is currently in
///
/// You must recite the bot's name thrice before it can be summoned (no titles)
///
/// **Usage:** `summon <name> <name> <name>`
#[poise::command(category = "join", prefix_command, slash_command, guild_only)]
pub async fn summon(
	ctx: Context<'_>,
	name_1: String,
	name_2: String,
	name_3: String,
) -> CommandResult {
	let name = ctx
		.discord()
		.cache
		.current_user_field(|user| user.name.rsplit(' ').next().unwrap().to_lowercase());

	if name == name_1.to_lowercase()
		&& name == name_2.to_lowercase()
		&& name == name_3.to_lowercase()
	{
		run(
			&ctx,
			generic::summon(ctx.discord(), ctx.guild_id(), ctx.author().id),
		)
		.await
	} else {
		run(&ctx, async {
			Err("Your request is met with silence".into())
		})
		.await
	}
}

/// Remove the bot from the voice channel it is in
///
/// **Usage:** `banish`
#[poise::command(category = "join", prefix_command, slash_command, guild_only)]
pub async fn banish(ctx: Context<'_>) -> CommandResult {
	run(&ctx, generic::banish(ctx.discord(), ctx.guild_id())).await
}
