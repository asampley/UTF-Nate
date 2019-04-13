use serenity::prelude::Context;
use serenity::framework::standard::{Args, Command, CommandError};
use serenity::model::channel::Message;

pub struct Unicode;

impl Command for Unicode {
    fn execute(&self, _: &mut Context, message: &Message, args: Args) -> Result<(), CommandError> {
        if args.len() != 1 {
            return Err(CommandError(format!("Expected 1 argument, got {}", args.len())))
        }

        let code_str = args.current().unwrap();
        let code =
            if code_str.starts_with("0x") {
                u32::from_str_radix(&code_str[2..], 16)
            } else {
                u32::from_str_radix(code_str, 10)
            };

        let reply = match code {
            Err(_err) => format!("Invalid character code: {}", code_str),
            Ok(c) => match std::char::from_u32(c) {
                None => format!("Invalid character code: {}", code_str),
                Some(c) => c.to_string(),
            }
        };

        match message.reply(&reply) {
            Err(err) => Err(CommandError(format!("Unable to send message: {:?}", err))),
            Ok(_) => Ok(()),
        }
    }
}
