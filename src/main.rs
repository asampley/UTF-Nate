mod configuration;
mod handler;
mod unicode;
mod voice;
mod data;
mod cmd;
mod util;

use serenity::client::Client;
use serenity::framework::standard::{
    Args,
    CommandGroup,
    CommandResult,
    DispatchError,
    help_commands,
    HelpOptions,
    StandardFramework,
};
use serenity::framework::standard::macros::help;
use serenity::model::id::UserId;
use serenity::model::channel::Message;
use serenity::prelude::Context;
use serenity::prelude::RwLock;

use util::check_msg;
use handler::Handler;
use unicode::UNICODE_GROUP;
use voice::VOICE_GROUP;
use data::{VoiceUserCache, VoiceManager, VoiceGuilds, ConfigResource};
use configuration::{Config, read_config};
use cmd::EXTERNAL_GROUP;

use std::sync::Arc;
use std::collections::HashSet;
use std::path::Path;

fn main() {
    // login with a bot token from an environment variable
    let mut client = Client::new(&read_token().expect("token"), Handler)
        .expect("Error creating client");

    {
        let mut data = client.data.write();
        // create voice manager to handle voice commands
        data.insert::<VoiceManager>(client.voice_manager.clone());
        data.insert::<VoiceUserCache>(Default::default());
        data.insert::<VoiceGuilds>(Default::default());
        data.insert::<ConfigResource>(Arc::new(RwLock::new(load_config())));
    }

    // create a framework to process message commands
    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix("!"))
            .before(before_hook)
            .after(after_hook)
            .help(&HELP)
            .group(&UNICODE_GROUP)
            .group(&VOICE_GROUP)
            .group(&EXTERNAL_GROUP)
            .unrecognised_command(unrecognised_command)
            .on_dispatch_error(on_dispatch_error)
    );

    if let Err(reason) = client.start() {
        eprintln!("An error occurred while running the client: {:?}", reason)
    }
}

fn read_token() -> std::io::Result<String> {
    std::fs::read_to_string("token")
}

fn load_config() -> Config {
    use configuration::Result::*;

    match read_config(Path::new("config.json")) {
        Ok(config) => {
            println!("Read config file from config.json");
            config
        },
        JsonError(reason) => {
            eprintln!("Error parsing config.json: {:?}", reason);
            println!("Creating default config");
            Config::default()
        },
        IoError(reason) => {
            eprintln!("Unable to access config.json: {:?}", reason);
            println!("Creating default config");
            Config::default()
        }
    }
}

#[help]
fn help(
    ctx: &mut Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>
) -> CommandResult {
    help_commands::plain(ctx, msg, args, help_options, groups, owners)
}

fn unrecognised_command(ctx: &mut Context, msg: &Message, cmd: &str) {
    let guild_name = msg.guild(&ctx.cache).map(|g| g.read().name.clone());
    check_msg(msg.reply(&ctx, format!("Unrecognised command: {}", cmd)));
    println!("User {} ({}) in guild {:?} ({:?}) command {} not recognised with message: {}",
        msg.author.name, msg.author.id,
        guild_name, msg.guild_id,
        cmd, msg.content
    );
}

fn before_hook(ctx: &mut Context, msg: &Message, cmd: &str)
    -> bool
{
    let guild_name = msg.guild(&ctx.cache).map(|g| g.read().name.clone());
    println!("User {} ({}) in guild {:?} ({:?}) running {} with message: {}",
        msg.author.name, msg.author.id,
        guild_name, msg.guild_id,
        cmd, msg.content);

    true
}

fn after_hook(ctx: &mut Context, msg: &Message, cmd: &str, res: CommandResult) {
    let guild_name = msg.guild(&ctx.cache).map(|g| g.read().name.clone());

    println!("User {} ({}) in guild {:?} ({:?}) completed {} with result {:?} with message: {}",
        msg.author.name, msg.author.id,
        guild_name, msg.guild_id,
        cmd, res, msg.content);
}

fn on_dispatch_error(ctx: &mut Context, msg: &Message, err: DispatchError) {
    use DispatchError::*;
    match err {
        NotEnoughArguments { min, given } => {
            let s = format!("Too few arguments. Expected at least {}, but got {}.", min, given);

            let _ = msg.channel_id.say(&ctx.http, &s);
        },
        TooManyArguments { max, given } => {
            let s = format!("Too many arguments. Expected at most {}, but got {}.", max, given);

            let _ = msg.channel_id.say(&ctx.http, &s);
        },
        _ => println!("Unhandled dispatch error: {:?}", err),
    }
}
