extern crate serenity;
mod handler;
mod unicode;
mod voice;

use serenity::client::Client;
use serenity::prelude::EventHandler;
use serenity::framework::standard::StandardFramework;

use handler::Handler;
use handler::VoiceCache;
use unicode::Unicode;
use voice::{Join, Leave, Play, VoiceManager};

use std::sync::Arc;
use std::collections::HashMap;

fn main() {
    // login with a bot token from an environment variable
    let mut client = Client::new(&read_token().expect("token"), Handler)
        .expect("Error creating client");

    {
        let mut data = client.data.lock();
        // create voice manager to handle voice commands
        data.insert::<VoiceManager>(Arc::clone(&client.voice_manager));
        data.insert::<VoiceCache>(HashMap::default());
    }

    // create a framework to process message commands
    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix("!"))
            .cmd("u", Unicode)
            .cmd("join", Join)
            .cmd("summon", Join)
            .cmd("leave", Leave)
            .cmd("banish", Leave)
            .cmd("play", Play)
    );

    if let Err(reason) = client.start() {
        eprintln!("An error occurred while running the client: {:?}", reason)
    }
}

fn read_token() -> std::io::Result<String> {
    std::fs::read_to_string("token")
}
