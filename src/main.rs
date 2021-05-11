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
use serenity::framework::standard::macros::{ help, hook };
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

#[tokio::main]
async fn main() {
    // login with a bot token from an environment variable
    let mut client = Client::builder(&read_token().expect("Token could not be read"))
        .event_handler(Handler)
        .framework(
            // create a framework to process message commands
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
        )
        .type_map_insert::<VoiceUserCache>(Default::default())
        .type_map_insert::<VoiceGuilds>(Default::default())
        .type_map_insert::<ConfigResource>(Arc::new(RwLock::new(load_config())))
        .await
        .expect("Error creating client");

    {
        let mut data = client.data.write().await;
        // create voice manager to handle voice commands
        data.insert::<VoiceManager>(client.voice_manager.clone());
    }

    if let Err(reason) = client.start().await {
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
async fn help(
    ctx: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>
) -> CommandResult {
    help_commands::plain(ctx, msg, args, help_options, groups, owners).await;
    Ok(())
}

#[hook]
async fn unrecognised_command(ctx: &Context, msg: &Message, cmd: &str) {
    let guild_name = msg.guild_field(&ctx.cache, |g| g.name.clone()).await;
    check_msg(msg.reply(&ctx, format!("Unrecognised command: {}", cmd)).await);
    println!("User {} ({}) in guild {:?} ({:?}) command {} not recognised with message: {}",
        msg.author.name, msg.author.id,
        guild_name, msg.guild_id,
        cmd, msg.content
    );
}

#[hook]
async fn before_hook(ctx: &Context, msg: &Message, cmd: &str)
    -> bool
{
    let guild_name = msg.guild_field(&ctx.cache, |g| g.name.clone()).await;
    println!("User {} ({}) in guild {:?} ({:?}) running {} with message: {}",
        msg.author.name, msg.author.id,
        guild_name, msg.guild_id,
        cmd, msg.content);

    true
}

#[hook]
async fn after_hook(ctx: &Context, msg: &Message, cmd: &str, res: CommandResult) {
    let guild_name = msg.guild_field(&ctx.cache, |g| g.name.clone()).await;

    println!("User {} ({}) in guild {:?} ({:?}) completed {} with result {:?} with message: {}",
        msg.author.name, msg.author.id,
        guild_name, msg.guild_id,
        cmd, res, msg.content);
}

#[hook]
async fn on_dispatch_error(ctx: &Context, msg: &Message, err: DispatchError) {
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
