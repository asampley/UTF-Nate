extern crate serenity;
extern crate serde;
extern crate serde_json;

mod configuration;
mod handler;
mod unicode;
mod voice;
mod data;

use serenity::client::Client;
use serenity::framework::standard::StandardFramework;
use serenity::model::channel::Message;
use serenity::prelude::Context;

use handler::Handler;
use unicode::Unicode;
use voice::{Join, Leave, Play, Volume, Stop, Intro, Outro, List};
use data::{VoiceUserCache, VoiceManager, VoiceGuilds, ConfigResource};
use configuration::{Config, read_config};

use std::sync::Arc;
use std::collections::HashMap;
use std::path::Path;

fn main() {
    // login with a bot token from an environment variable
    let mut client = Client::new(&read_token().expect("token"), Handler)
        .expect("Error creating client");

    {
        let mut data = client.data.lock();
        // create voice manager to handle voice commands
        data.insert::<VoiceManager>(Arc::clone(&client.voice_manager));
        data.insert::<VoiceUserCache>(HashMap::default());
        data.insert::<VoiceGuilds>(HashMap::default());
        data.insert::<ConfigResource>(load_config());
    }

    // create a framework to process message commands
    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix("!"))
            .before(before_hook)
            .cmd("u", Unicode)
            .cmd("join", Join)
            .cmd("summon", Join)
            .cmd("leave", Leave)
            .cmd("banish", Leave)
            .cmd("play", Play)
            .cmd("volume", Volume)
            .cmd("stop", Stop)
            .cmd("intro", Intro)
            .cmd("outro", Outro)
            .cmd("playlist", List)
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

fn before_hook(_ctx: &mut Context, msg: &Message, cmd: &str)
    -> bool
{
    let guild_name = msg.guild().map(|g| g.read().name.clone());
    println!("User {} ({}) in guild {:?} ({:?}) running {} with message: {}",
        msg.author.name, msg.author.id,
        guild_name, msg.guild_id,
        cmd, msg.content);

    true
}
