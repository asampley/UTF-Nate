use serenity::model::id::UserId;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

#[derive(Default, Serialize, Deserialize)]
pub struct Config {
    pub intros: HashMap<UserId, String>,
    pub outros: HashMap<UserId, String>,
}

pub enum Result<T> {
    Ok(T),
    JsonError(serde_json::Error),
    IoError(std::io::Error),
}

pub fn write_config(path: &Path, config: &Config) -> Result<()> {
    let file = match File::create(path) {
        Err(err) => return Result::IoError(err),
        Ok(file) => file,
    };

    match serde_json::to_writer(file, config) {
        Err(err) => Result::JsonError(err),
        Ok(()) => Result::Ok(()),
    }
}

pub fn read_config(path: &Path) -> Result<Config> {
    let file = match File::open(path) {
        Err(err) => return Result::IoError(err),
        Ok(file) => file,
    };

    match serde_json::from_reader(file) {
        Err(err) => Result::JsonError(err),
        Ok(config) => Result::Ok(config),
    }
}
