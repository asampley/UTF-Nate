use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;
use serenity::model::interactions::application_command::{
	ApplicationCommandInteraction, ApplicationCommandOptionType,
};

use crate::commands::{create_interaction_set_description, run};
use crate::util::*;

mod generic;

#[group("roll")]
#[description("Commands for rolling dice")]
#[commands(roll)]
pub struct Roll;

pub async fn roll_interaction(
	ctx: &Context,
	int: &ApplicationCommandInteraction,
) -> serenity::Result<()> {
	let expression = match get_option_string(ctx, int, &int.data.options, "expression").await {
		Ok(value) => value.map(|s| s.to_string()),
		Err(result) => return result,
	};

	run(ctx, int, generic::roll(expression.unwrap_or_default())).await
}

pub fn roll_interaction_create(
	cmd: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
	create_interaction_set_description(&ROLL_COMMAND, cmd, "Roll some dice and do some math")
		.create_option(|option| {
			option
				.name("expression")
				.description("Dice expression to roll and calculate")
				.kind(ApplicationCommandOptionType::String)
				.required(true)
		})
}

#[command]
#[help_available]
#[description("
	Roll a die or dice

	xdy [OPTIONS] [TARGET] [FAILURE] [! REASON]

	roll `x` dice(s) with `y` sides

	`y` can also be \"F\" or \"f\" for fudge dice. In this case, no option applies and ignored if provided.

	Options:
	+ - / * : modifiers
	e# : Explode value. If number is omitted, we use dice sides
	ie# or !# : Indefinite explode value, If number is omitted, we use dice sides
	K#  : Keeping # highest (upperacse \"K\")
	k#  : Keeping # lowest (lowercase \"k\")
	D#  : Dropping the highest (uppercase \"D\")
	d#  : Dropping the lowest (lowercase \"d\")
	r#  : Reroll if <= value
	ir# : Indefinite reroll if <= value

	Target:
	t#  : minimum value to count as success
	tt# : minimum value to count as two successes
	t[<list of numbers>] : enumeration of values considered as success

	Failure:
	f# : value under which it's counted as failure

	Repetition:
	a roll can be repeated with `^` operator: `(2d6 + 6) ^ 8` will roll eight times the expression.

	Summed repetition:
	with the `^+` operator, the roll will be repeated and all the totals summed.

	Sorted repetition:
	with the `^#` operator, the roll will be repeated and sorted by total.

	Reason:
	: : Any text after `:` will be a comment
")]
#[min_args(1)]
#[usage("<expression>")]
#[example("2d6 + 3d10")]
#[example("3d6 + 5")]
#[example("3d6 * 1.5")]
#[example("3d6 e6")]
#[example("3d6 ie6")]
#[example("3d6!")]
#[example("3d10 K2")]
#[example("4d6 r2")]
#[example("4d6 ir2")]
#[example("6d10 t7")]
pub async fn roll(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
	run(ctx, msg, generic::roll(args.rest().to_string())).await
}
