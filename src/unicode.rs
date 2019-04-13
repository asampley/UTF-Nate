use serenity::prelude::Context;
use serenity::framework::standard::{Args, Command, CommandError};
use serenity::model::channel::Message;
use std::num::ParseIntError;

pub struct Unicode;

enum ParseCodeError {
    ParseIntError(ParseIntError),
    InvalidCode(u32),
}

impl Unicode {
    fn parse_code(string: &str) -> Result<char, ParseCodeError> {
        use ParseCodeError::*;

        let code = if string.starts_with("0x") {
            u32::from_str_radix(&string[2..], 16)
        } else {
            u32::from_str_radix(string, 10)
        };

        match code {
            Err(parse_error) => Err(ParseIntError(parse_error)),
            Ok(c) => match std::char::from_u32(c) {
                None => Err(InvalidCode(c)),
                Some(c) => Ok(c),
            }
        }
    }
}

impl Command for Unicode {
    fn execute(&self, _: &mut Context, message: &Message, mut args: Args)
        -> Result<(), CommandError>
    {
        let mut chars = Vec::with_capacity(args.len());
        let mut reply = None;

        for arg in args.iter::<String>() {
            let code_str = arg.unwrap();
            let code = Unicode::parse_code(&code_str);

            let c = match code {
                Err(_) => {
                    reply = Some(format!("Invalid character code: {}", code_str));
                    break;
                },
                Ok(c) => c,
            };

            chars.push(c);
        }

        if reply.is_none() {
            reply = Some(chars.into_iter().collect());
        }

        match message.reply(&reply.unwrap()) {
            Err(err) => Err(CommandError(format!("Unable to send message: {:?}", err))),
            Ok(_) => Ok(()),
        }
    }
}
