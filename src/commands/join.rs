use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;
use serenity::model::interactions::application_command::ApplicationCommandInteraction;

use crate::commands::{create_interaction, run};

mod generic;

#[group("join")]
#[description("Commands to move the bot around voice channels")]
#[commands(summon, banish)]
pub struct Join;

#[command]
#[only_in(guilds)]
#[help_available]
#[description("Summon the bot to the voice channel the user is currently in")]
pub async fn summon(ctx: &Context, msg: &Message) -> CommandResult {
	run(ctx, msg, generic::summon(ctx, msg.guild_id, msg.author.id)).await
}

pub async fn summon_interaction(
	ctx: &Context,
	int: &ApplicationCommandInteraction,
) -> serenity::Result<()> {
	run(ctx, int, generic::summon(ctx, int.guild_id, int.user.id)).await
}

pub fn summon_interaction_create(
	cmd: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
	create_interaction(&SUMMON_COMMAND, cmd)
}

#[command]
#[only_in(guilds)]
#[help_available]
#[description("Remove the bot from the voice channel it is in")]
pub async fn banish(ctx: &Context, msg: &Message) -> CommandResult {
	run(ctx, msg, generic::banish(ctx, msg.guild_id)).await
}

pub async fn banish_interaction(
	ctx: &Context,
	int: &ApplicationCommandInteraction,
) -> serenity::Result<()> {
	run(ctx, int, generic::banish(ctx, int.guild_id)).await
}

pub fn banish_interaction_create(
	cmd: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
	create_interaction(&BANISH_COMMAND, cmd)
}
