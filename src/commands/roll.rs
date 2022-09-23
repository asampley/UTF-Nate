use crate::commands::run;
use crate::util::*;

mod generic;

/// Roll a die or dice
///
/// xdy [OPTIONS] [TARGET] [FAILURE] [! REASON]
///
/// roll `x` dice(s) with `y` sides
///
/// `y` can also be \"F\" or \"f\" for fudge dice. In this case, no option applies and ignored if provided.
///
/// Options:
/// + - / * : modifiers
/// e# : Explode value. If number is omitted, we use dice sides
/// ie# or !# : Indefinite explode value, If number is omitted, we use dice sides
/// K#  : Keeping # highest (upperacse \"K\")
/// k#  : Keeping # lowest (lowercase \"k\")
/// D#  : Dropping the highest (uppercase \"D\")
/// d#  : Dropping the lowest (lowercase \"d\")
/// r#  : Reroll if <= value
/// ir# : Indefinite reroll if <= value
///
/// Target:
/// t#  : minimum value to count as success
/// tt# : minimum value to count as two successes
/// t[<list of numbers>] : enumeration of values considered as success
///
/// Failure:
/// f# : value under which it's counted as failure
///
/// Repetition:
/// a roll can be repeated with `^` operator: `(2d6 + 6) ^ 8` will roll eight times the expression.
///
/// Summed repetition:
/// with the `^+` operator, the roll will be repeated and all the totals summed.
///
/// Sorted repetition:
/// with the `^#` operator, the roll will be repeated and sorted by total.
///
/// Reason:
/// : : Any text after `:` will be a comment
///
/// **Usage:** `roll <expression>`
/// - `roll 2d6 + 3d10`
/// - `roll 3d6 + 5`
/// - `roll 3d6 * 1.5`
/// - `roll 3d6 e6`
/// - `roll 3d6 ie6`
/// - `roll 3d6!`
/// - `roll 3d10 K2`
/// - `roll 4d6 r2`
/// - `roll 4d6 ir2`
/// - `roll 6d10 t7`
#[poise::command(category = "roll", prefix_command, slash_command)]
pub async fn roll(
	ctx: Context<'_>,
	#[description = "Dice expression to roll and calculate"]
	#[rest]
	expression: String,
) -> CommandResult {
	run(&ctx, generic::roll(expression)).await
}
