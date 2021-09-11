use serenity::builder::CreateApplicationCommand;
use serenity::framework::standard::Command;

pub fn create_interaction<'a>(
	cmd: &Command,
	create: &'a mut CreateApplicationCommand,
) -> &'a mut CreateApplicationCommand {
	let opt = cmd.options;

	create.name(opt.names[0]);
	opt.desc.map(|d| create.description(d));

	create
}
