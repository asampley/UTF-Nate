use tracing::{debug, info};

use poise::Command;

use serenity::http::client::Http;
use serenity::model::application::command::Command as SerenityCommand;

pub async fn reregister<U, E>(
	http: impl AsRef<Http>,
	commands: &[Command<U, E>],
) -> serenity::Result<()> {
	info!("Reregistering slash commands...");

	let http = http.as_ref();

	let old_commands = http.get_global_application_commands().await?;

	for old_command in old_commands {
		http.delete_global_application_command(old_command.id.into())
			.await
			.unwrap();
	}
	info!("Deleted old slash commands");

	let create_commands = poise::builtins::create_application_commands(commands);

	SerenityCommand::set_global_application_commands(http, |c| {
		*c = create_commands;
		c
	})
	.await
	.unwrap();

	debug!(
		"Registered slash commands: {:#?}",
		SerenityCommand::get_global_application_commands(&http).await?,
	);
	info!("Reregistered slash commands");

	Ok(())
}
