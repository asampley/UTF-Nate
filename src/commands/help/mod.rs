use markdown::mdast::Node;

use ::poise::{Command, FrameworkContext};

use std::fmt::Write;

use crate::commands::CustomData;
use crate::util::Response;

pub mod poise;

pub const fn help_help() -> &'static str {
	include_str!("help/help.md")
}

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

	if let Some(data) = command.custom_data.downcast_ref::<CustomData>() {
		writeln!(text, "\n{}", md_discord((data.help_md)())).unwrap();
	}
}

pub fn write_groups<U, E>(text: &mut String, commands: &[Command<U, E>]) {
	let mut commands = commands
		.iter()
		.map(|command| {
			(
				command.category.as_deref(),
				&command.qualified_name,
				command
					.custom_data
					.downcast_ref::<CustomData>()
					.and_then(|d| (d.help_md)().lines().next()),
			)
		})
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
		.filter(|c| c.name == name[0] || c.aliases.contains(&name[0]))
		.find_map(|c| {
			if name.len() == 1 {
				Some(c)
			} else {
				find_command(&c.subcommands, &name[1..])
			}
		})
}

pub fn md_discord(md: &str) -> String {
	let tokens = markdown::to_mdast(
		md,
		&markdown::ParseOptions {
			// ignore most constructs that discord would preserve
			constructs: markdown::Constructs {
				attention: false,
				autolink: false,
				block_quote: false,
				character_escape: true,
				character_reference: true,
				code_indented: false,
				code_fenced: false,
				code_text: false,
				definition: false,
				frontmatter: false,
				gfm_autolink_literal: false,
				gfm_label_start_footnote: false,
				gfm_footnote_definition: false,
				gfm_strikethrough: false,
				gfm_table: false,
				gfm_task_list_item: false,
				hard_break_escape: true,
				hard_break_trailing: true,
				heading_atx: false,
				heading_setext: false,
				html_flow: false,
				html_text: false,
				label_start_image: false,
				label_start_link: false,
				label_end: false,
				list_item: false,
				math_flow: false,
				math_text: false,
				mdx_esm: false,
				mdx_expression_flow: false,
				mdx_expression_text: false,
				mdx_jsx_flow: false,
				mdx_jsx_text: false,
				thematic_break: false,
			},
			..Default::default()
		},
	)
	.unwrap();

	let mut string = String::new();

	md_discord_format(&mut string, &[tokens]).unwrap();

	string
}

// TODO escape markdown that is contained in text
fn md_discord_format(f: &mut String, nodes: &[Node]) -> std::fmt::Result {
	use Node::*;

	for node in nodes {
		match node {
			// ignored constructs
			BlockQuote(_)
			| FootnoteDefinition(_)
			| MdxJsxFlowElement(_)
			| List(_)
			| MdxjsEsm(_)
			| Toml(_)
			| Yaml(_)
			| InlineCode(_)
			| InlineMath(_)
			| Delete(_)
			| Emphasis(_)
			| MdxTextExpression(_)
			| FootnoteReference(_)
			| Html(_)
			| Image(_)
			| ImageReference(_)
			| MdxJsxTextElement(_)
			| Link(_)
			| LinkReference(_)
			| Strong(_)
			| Code(_)
			| Math(_)
			| MdxFlowExpression(_)
			| Heading(_)
			| Table(_)
			| ThematicBreak(_)
			| TableRow(_)
			| TableCell(_)
			| ListItem(_)
			| Definition(_) => unreachable!(),
			Root(x) => md_discord_format(f, &x.children),
			Paragraph(x) => {
				md_discord_format(f, &x.children)?;
				write!(f, "\n\n")
			}
			Break(_) => writeln!(f),
			Text(x) => write!(f, "{}", x.value),
		}?
	}

	Ok(())
}
