use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;
use serenity::model::interactions::application_command::{
	ApplicationCommandInteraction, ApplicationCommandOptionType,
};

use crate::commands::{create_interaction, run};
use crate::util::*;

mod generic;

#[group("help")]
#[description("Display information about how to use the bot")]
#[commands(help)]
pub struct Help;

#[command]
#[help_available]
#[description("Display information about all commands or specific commands")]
#[usage("<command?>")]
#[example("")]
#[example("play")]
pub async fn help(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	run(ctx, msg, generic::help(args.current())).await
}

pub fn help_interaction_create(
	cmd: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
	create_interaction(&HELP_COMMAND, cmd).create_option(|option| {
		option
			.name("name")
			.description("Name of command to get help for")
			.kind(ApplicationCommandOptionType::String)
	})
}

pub async fn help_interaction(
	ctx: &Context,
	int: &ApplicationCommandInteraction,
) -> serenity::Result<()> {
	let name = get_option_string(ctx, int, &int.data.options, "name").await?;

	run(ctx, int, generic::help(name)).await
}
