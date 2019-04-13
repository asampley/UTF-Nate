#[macro_use] extern crate serenity;

use serenity::client::Client;
use serenity::prelude::EventHandler;
use serenity::framework::standard::StandardFramework;

struct Handler;

// implement default event handler
impl EventHandler for Handler {}

fn main() {
    // login with a bot token from an environment variable
    let mut client = Client::new(&read_token().expect("token"), Handler)
        .expect("Error creating client");

    // create a framework to process message commands
    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix("!"))
            .cmd("ping", Ping)
    );

    if let Err(reason) = client.start() {
        eprintln!("An error occurred while running the client: {:?}", reason)
    }
}

command!(Ping(_context, message) {
    let _ = message.reply("Pong!");
});

fn read_token() -> std::io::Result<String> {
    std::fs::read_to_string("token")
}
