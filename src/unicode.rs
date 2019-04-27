use serenity::prelude::Context;
use serenity::framework::standard::{Args, Command, CommandError, CommandOptions};
use serenity::model::channel::Message;
use std::num::ParseIntError;
use std::sync::Arc;

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
    fn options(&self) -> Arc<CommandOptions> {
        Arc::new(CommandOptions {
            help_available: true,
            desc: Some(String::from("Print the characters based on the unicode code point. The code point can be specified in either decimal or hexidecimal (by preceding it with 0x).")),
            usage: Some(String::from("[codepoint...]")),
            example: Some(String::from("0x252C 0x2500 0x252C 0x30CE 0x28 0x20 0xBA 0x20 0x5F 0x20 0xBA 0x30CE 0x29")),
            ..Default::default()
        })
    }

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
