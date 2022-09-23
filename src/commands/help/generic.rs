use poise::{Command, FrameworkContext};

use std::fmt::Write;

use crate::util::Response;

#[tracing::instrument(level = "info", ret, skip(framework))]
pub async fn help<U, E>(
	command_name: &[String],
	framework: FrameworkContext<'_, U, E>,
) -> Result<Response, Response> {
	match command_name {
		[] => {
			let mut s = "Pass the name of any command to get more details\n".into();

			write_groups(&mut s, &framework.options.commands);

			Ok(s.into())
		}
		_ => {
			let command = find_command(&framework.options.commands, command_name);

			match command {
				Some(command) => {
					let mut s = String::new();

					write_command(&mut s, command);

					Ok(s.into())
				}
				None => Err("No command with that name found. \
					Use help with no arguments to see a list of commands."
					.into()),
			}
		}
	}
}

pub fn write_command<U, E>(text: &mut String, command: &Command<U, E>) {
	let name = &command.qualified_name;

	writeln!(text, "__**{}**__", name).unwrap();

	if let Some(desc) = &command.description {
		writeln!(text, "\n{}", desc).unwrap();
	}

	if let Some(help_text) = command.help_text {
		writeln!(text, "\n{}", help_text()).unwrap();
	}
}

pub fn write_groups<U, E>(text: &mut String, commands: &[Command<U, E>]) {
	let mut commands = commands
		.iter()
		.map(|command| (
            command.category,
            &command.qualified_name,
            &command.description,
		))
		.collect::<Vec<_>>();

	commands.sort();

	let mut heading = None;

	for (cat, name, desc) in commands {
		if heading != Some(cat) {
			heading = Some(cat);

			writeln!(text, "\n__**{}:**__", cat.unwrap_or("uncategorized")).unwrap();
		}

		write!(text, "  `{}`", name).unwrap();

		if let Some(desc) = desc {
			write!(text, " {}", desc).unwrap();
		}

		writeln!(text).unwrap();
	}
}

pub fn find_command<'a, U, E>(
	commands: &'a [Command<U, E>],
	name: &[String],
) -> Option<&'a Command<U, E>> {
	commands
		.iter()
		.filter(|c| c.name == name[0] || c.aliases.contains(&name[0].as_str()))
		.filter_map(|c| {
			if name.len() == 1 {
				Some(c)
			} else {
				find_command(&c.subcommands, &name[1..])
			}
		})
		.next()
}
