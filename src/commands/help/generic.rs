use itertools::Itertools;

use serenity::framework::standard::{Command, CommandGroup};

use std::fmt::Write;

use crate::util::Response;
use crate::GROUPS;

#[tracing::instrument(level = "info")]
pub async fn help(name: Option<&str>) -> Result<Response, Response> {
	match name {
		Some(name) => {
			let command = find_command(GROUPS, name);

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
		None => {
			let mut s = "Pass the name of any command to get more details\n\n".into();

			write_groups(&mut s, GROUPS, 0);

			Ok(s.into())
		}
	}
}

pub fn write_command(text: &mut String, command: &Command) {
	let name = command.options.names[0];

	writeln!(text, "__**{}**__\n", name).unwrap();

	if let Some(desc) = command.options.desc {
		writeln!(text, "{}", desc).unwrap();
	}

	if let Some(usage) = command.options.usage {
		writeln!(text, "**Usage:** `{} {}`", name, usage).unwrap();
	}

	for example in command.options.examples.iter() {
		writeln!(text, "**Example:** `{} {}`", name, example).unwrap();
	}
}

pub fn write_groups(text: &mut String, groups: &[&CommandGroup], nest_level: usize) {
	let indent = "  ".repeat(nest_level);

	for group in groups {
		writeln!(
			text,
			"{}__**{}:**__ `{}`",
			indent,
			group.name,
			group
				.options
				.commands
				.iter()
				.map(|c| c.options.names[0])
				.join("`, `"),
		)
		.unwrap();

		write_groups(text, group.options.sub_groups, nest_level + 1);
	}
}

pub fn find_command(groups: &[&CommandGroup], name: &str) -> Option<&'static Command> {
	groups
		.iter()
		.filter_map(|group| {
			group
				.options
				.commands
				.iter()
				.filter(|c| c.options.names.iter().any(|n| n == &name))
				.next()
				.map(|v| *v)
				.or_else(|| find_command(group.options.sub_groups, name))
		})
		.next()
}
