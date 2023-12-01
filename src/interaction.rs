//! Code to register interations (slash commands) with discord.

use tracing::{debug, info};

use poise::Command;

use serenity::http::Http;
use serenity::model::application::Command as SerenityCommand;

/// Reregister all of the slash commands with Discord. If this function isn't
/// called then the slash commands will not appear as commands to users.
pub async fn reregister<U, E>(
	http: impl AsRef<Http>,
	commands: &[Command<U, E>],
) -> serenity::Result<()> {
	info!("Reregistering slash commands...");

	let http = http.as_ref();

	let old_commands = http.get_global_commands().await?;

	for old_command in old_commands {
		http.delete_global_command(old_command.id).await.unwrap();
	}
	info!("Deleted old slash commands");

	let create_commands = commands
		.iter()
		.filter_map(|c| c.create_as_slash_command())
		.collect();

	SerenityCommand::set_global_commands(http, create_commands)
		.await
		.unwrap();

	debug!(
		"Registered slash commands: {:#?}",
		SerenityCommand::get_global_commands(&http).await?,
	);
	info!("Reregistered slash commands");

	Ok(())
}
