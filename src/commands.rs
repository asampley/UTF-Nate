//! All commands are defined under this module.
//!
//! Commands can be created for adding to the bot using [`commands()`].

pub mod external;
pub mod help;
pub mod herald;
pub mod join;
pub mod play;
pub mod queue;
pub mod roll;
pub mod unicode;
pub mod voice;

use futures::Future;

use crate::util::{Command, CommandResult, Context, Respond, Response};

/// Create a vector containing all the commands.
pub fn commands() -> Vec<Command> {
	vec![
		external::cmd(),
		external::cmdlist(),
		help::help(),
		herald::intro(),
		herald::introbot(),
		herald::outro(),
		join::summon(),
		join::banish(),
		play::clip(),
		play::play(),
		play::playnext(),
		play::playnow(),
		queue::stop(),
		queue::skip(),
		queue::pause(),
		queue::unpause(),
		queue::queue(),
		queue::shuffle(),
		queue::shufflenow(),
		queue::r#loop(),
		roll::roll(),
		unicode::unicode(),
		voice::volume(),
		voice::list(),
	]
}

/// Await a command `f`, and then reply to the initiating message with the
/// response from the command.
pub async fn run<F>(ctx: &Context<'_>, f: F) -> CommandResult
where
	F: Future<Output = Result<Response, Response>>,
{
	ctx.respond(f.await.as_ref()).await?;

	Ok(())
}
