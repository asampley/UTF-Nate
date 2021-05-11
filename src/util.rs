use serenity::Result;
use serenity::model::channel::Message;

use std::path::Path;

pub fn sandboxed_exists(sandbox: &Path, path: &Path) -> bool {
    match dbg!(dbg!(sandbox).canonicalize()) {
        Ok(sandbox) => match dbg!(dbg!(path).canonicalize()) {
            Ok(path) => path.ancestors().any(|d| d == sandbox) && dbg!(path.exists()),
            Err(_) => false,
        }
        Err(_) => false,
    }
}

pub fn check_msg(result: Result<Message>) {
    if let Err(reason) = result {
        eprintln!("Error sending message: {:?}", reason);
    }
}
