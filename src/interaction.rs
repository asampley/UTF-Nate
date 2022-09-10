use tracing::{debug, info};

use serenity::http::client::Http;
use serenity::model::application::command::Command;

use crate::commands::external::{cmd_interaction_create, cmdlist_interaction_create};
use crate::commands::help::help_interaction_create;
use crate::commands::herald::{
	intro_interaction_create, introbot_interaction_create, outro_interaction_create,
};
use crate::commands::join::{banish_interaction_create, summon_interaction_create};
use crate::commands::play::{
	clip_interaction_create, play_interaction_create, playnext_interaction_create,
	playnow_interaction_create,
};
use crate::commands::queue::{
	loop_interaction_create, pause_interaction_create, queue_interaction_create,
	shuffle_interaction_create, shufflenow_interaction_create, skip_interaction_create,
	stop_interaction_create, unpause_interaction_create,
};
use crate::commands::roll::roll_interaction_create;
use crate::commands::voice::{list_interaction_create, volume_interaction_create};

pub async fn reregister(http: &Http) -> serenity::Result<()> {
	info!("Reregistering slash commands...");

	let commands = http.get_global_application_commands().await?;

	for command in commands {
		http.delete_global_application_command(command.id.into())
			.await
			.unwrap();
	}
	info!("Deleted old slash commands");

	Command::set_global_application_commands(http, |commands| {
		commands
			.create_application_command(intro_interaction_create)
			.create_application_command(outro_interaction_create)
			.create_application_command(introbot_interaction_create)
			.create_application_command(summon_interaction_create)
			.create_application_command(banish_interaction_create)
			.create_application_command(list_interaction_create)
			.create_application_command(clip_interaction_create)
			.create_application_command(play_interaction_create)
			.create_application_command(playnext_interaction_create)
			.create_application_command(playnow_interaction_create)
			.create_application_command(volume_interaction_create)
			.create_application_command(stop_interaction_create)
			.create_application_command(skip_interaction_create)
			.create_application_command(pause_interaction_create)
			.create_application_command(unpause_interaction_create)
			.create_application_command(queue_interaction_create)
			.create_application_command(shuffle_interaction_create)
			.create_application_command(shufflenow_interaction_create)
			.create_application_command(loop_interaction_create)
			.create_application_command(help_interaction_create)
			.create_application_command(roll_interaction_create)
			.create_application_command(cmd_interaction_create)
			.create_application_command(cmdlist_interaction_create)
	})
	.await
	.unwrap();

	debug!(
		"Registered slash commands: {:#?}",
		Command::get_global_application_commands(&http).await?,
	);
	info!("Reregistered slash commands");

	Ok(())
}
