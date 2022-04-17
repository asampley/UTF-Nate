pub mod cmd;
pub mod help;
pub mod herald;
pub mod join;
pub mod play;
pub mod queue;
pub mod unicode;
pub mod voice;
pub mod roll;

use futures::Future;

use serenity::builder::CreateApplicationCommand;
use serenity::framework::standard::Command;
use serenity::prelude::Context;

use crate::util::{Respond, Response};

pub async fn run<R, F, E>(ctx: &Context, rsp: &R, f: F) -> Result<(), E>
where
	R: Respond,
	F: Future<Output = Result<Response, Response>>,
	E: From<serenity::Error>,
{
	rsp.respond(ctx, f.await.as_ref()).await?;

	Ok(())
}

pub fn create_interaction<'a>(
	cmd: &Command,
	create: &'a mut CreateApplicationCommand,
) -> &'a mut CreateApplicationCommand {
	let opt = cmd.options;

	create.name(opt.names[0]);
	opt.desc.map(|d| create.description(d));

	create
}

pub fn create_interaction_set_description<'a>(
	cmd: &Command,
	create: &'a mut CreateApplicationCommand,
	desc: &str,
) -> &'a mut CreateApplicationCommand {
	let opt = cmd.options;

	create.name(opt.names[0]);
	create.description(desc);

	create
}
