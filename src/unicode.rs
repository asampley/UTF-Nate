use serenity::prelude::Context;
use serenity::framework::standard::{Args, CommandResult, CommandError};
use serenity::framework::standard::macros::{command, group};
use serenity::model::channel::Message;
use std::num::ParseIntError;

enum ParseCodeError {
    ParseIntError(ParseIntError),
    InvalidCode(u32),
}

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

#[group("unicode")]
#[description("Some fun with unicode")]
#[commands(unicode)]
struct Unicode;

#[command]
#[aliases("u")]
#[help_available]
#[description("Print the characters based on the unicode code point. The code point can be specified in either decimal or hexidecimal (by preceding it with 0x).")]
#[usage("[codepoint...]")]
#[example("0x252C 0x2500 0x252C 0x30CE 0x28 0x20 0xBA 0x20 0x5F 0x20 0xBA 0x30CE 0x29")]
pub fn unicode(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut chars = Vec::with_capacity(args.len());
    let mut reply = None;

    for arg in args.iter::<String>() {
        let code_str = arg.unwrap();
        let code = parse_code(&code_str);

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

    match msg.reply(&ctx, &reply.unwrap()) {
        Err(err) => Err(CommandError(format!("Unable to send message: {:?}", err))),
        Ok(_) => Ok(()),
    }
}
