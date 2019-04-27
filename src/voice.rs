use serenity::framework::standard::{Args, Command, CommandError, CommandOptions};
use serenity::model::channel::Message;
use serenity::client::Context;
use serenity::voice;
use serenity::Result as SerenityResult;
use serenity::voice::AudioSource;

use std::fs::read_dir;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use itertools::Itertools;

use crate::data::{VoiceManager, VoiceGuilds, VoiceGuild, ConfigResource};
use crate::configuration;
use crate::configuration::write_config;

fn clip_path() -> PathBuf {
    return Path::new("./resources/clips").canonicalize().unwrap();
}

pub struct Join;
pub struct Leave;
pub struct Play;
pub struct Volume;
pub struct Stop;
pub struct Intro;
pub struct Outro;
pub struct BotIntro;
pub struct List;

fn check_msg(result: SerenityResult<Message>) {
    if let Err(reason) = result {
        eprintln!("Error sending message: {:?}", reason);
    }
}

pub fn audio_source(loc: &str) -> serenity::Result<Box<dyn AudioSource>> {
    if loc.starts_with("http") {
        return voice::ytdl(&loc)
    } else {
        match get_clip(&loc) {
            Some(clip) => voice::ffmpeg(&clip),
            None => Err(serenity::Error::Other("Could not find source")),
        }
    }
}

fn get_clip(loc: &str) -> Option<PathBuf> {
    let clip_path = clip_path();
    let mut play_path = clip_path.join(&loc);

    for ext in &["mp3", "wav"] {
        play_path.set_extension(ext);

        if valid_clip(&play_path) {
            return Some(play_path);
        }
    }

    None
}

fn valid_clip(path: &Path) -> bool {
    let root_path = clip_path();

    match path.canonicalize() {
        Ok(path) => path.exists() && sandboxed(&root_path, &path),
        Err(_) => false,
    }
}

fn sandboxed(sandbox: &Path, path: &Path) -> bool {
    match sandbox.canonicalize() {
        Ok(sandbox) => match path.canonicalize() {
            Ok(path) => path.ancestors().any(|d| d == sandbox),
            Err(_) => false,
        }
        Err(_) => false,
    }
}

impl Command for Join {
    fn options(&self) -> Arc<CommandOptions> {
        Arc::new(CommandOptions {
            help_available: true,
            desc: Some(String::from("Summon the bot to the voice channel the user is currently in")),
            ..Default::default()
        })
    }

    fn execute(&self, ctx: &mut Context, msg: &Message, _: Args)
        -> Result<(), CommandError>
    {
        let guild = match msg.guild() {
            Some(guild) => guild,
            None => {
                check_msg(msg.channel_id.say("Groups and DMs not supported"));
                return Ok(());
            },
        };

        let guild_id = guild.read().id;

        let channel_id = guild
            .read()
            .voice_states.get(&msg.author.id)
            .and_then(|voice_state| voice_state.channel_id);

        let connect_to = match channel_id {
            Some(channel) => channel,
            None => {
                check_msg(msg.reply("Not in a voice channel"));
                return Ok(());
            }
        };

        let manager_lock = ctx.data.lock().get::<VoiceManager>().cloned().expect("Expected VoiceManager in ShareMap");
        let mut manager = manager_lock.lock();

        if manager.join(guild_id, connect_to).is_none() {
            check_msg(msg.channel_id.say("Error joining the channel"));
        }

        Ok(())
    }
}

impl Command for Leave {
    fn options(&self) -> Arc<CommandOptions> {
        Arc::new(CommandOptions {
            help_available: true,
            desc: Some(String::from("Remove the bot from the voice channel it is in")),
            ..Default::default()
        })
    }

    fn execute(&self, ctx: &mut Context, msg: &Message, _: Args)
        -> Result<(), CommandError>
    {
        let guild = match msg.guild() {
            Some(guild) => guild,
            None => {
                check_msg(msg.channel_id.say("Groups and DMs not supported"));
                return Ok(());
            }
        };

        let guild_id = guild.read().id;

        let manager_lock = ctx.data.lock().get::<VoiceManager>().cloned().expect("Expected VoiceManager in ShareMap");
        let mut manager = manager_lock.lock();
        let has_handler = manager.get(guild_id).is_some();

        if has_handler {
            manager.remove(guild_id);
        } else {
            check_msg(msg.reply("Not in a voice channel"));
        }

        Ok(())
    }
}

impl Command for Play {
    fn options(&self) -> Arc<CommandOptions> {
        Arc::new(CommandOptions {
            help_available: true,
            desc: Some(String::from("Play the specified clip")),
            usage: Some(String::from("<clip>")),
            example: Some(String::from("bnw/needoffspring")),
            ..Default::default()
        })
    }

    fn execute(&self, ctx: &mut Context, msg: &Message, mut args: Args)
        -> Result<(), CommandError>
    {
        let loc = match args.single::<String>() {
            Ok(loc) => loc,
            Err(_) => {
                check_msg(msg.channel_id.say("Must provide a source"));
                return Ok(())
            },
        };

        let guild = match msg.guild() {
            Some(guild) => guild,
            None => {
                check_msg(msg.channel_id.say("Groups and DMs not supported"));
                return Ok(());
            }
        };

        let guild_id = guild.read().id;

        {
            let mut data_lock = ctx.data.lock();

            let manager_lock = data_lock.get::<VoiceManager>().cloned().expect("Expected VoiceManager in ShareMap");
            let mut manager = manager_lock.lock();

            let voice_guild = data_lock.get_mut::<VoiceGuilds>()
                .expect("Expected VoiceGuilds in ShareMap")
                .entry(guild_id)
                .or_insert(VoiceGuild::default());

            if let Some(handler) = manager.get_mut(guild_id) {
                let source = audio_source(&loc);

                match source {
                    Ok(source) => {
                        voice_guild.add_audio(handler.play_returning(source));
                    },
                    Err(reason) => {
                        eprintln!("Error trying to play clip: {:?}", reason);
                        check_msg(msg.channel_id.say("Invalid clip"));
                    }
                }
            } else {
                check_msg(msg.channel_id.say("Not in a voice channel"));
            }
        }

        Ok(())
    }
}

impl Command for Volume {
    fn options(&self) -> Arc<CommandOptions> {
        Arc::new(CommandOptions {
            help_available: true,
            desc: Some(String::from("Stop all clips currently being played by the bot")),
            usage: Some(String::from("<volume>")),
            example: Some(String::from("0.5")),
            ..Default::default()
        })
    }

    fn execute(&self, ctx: &mut Context, msg: &Message, mut args: Args)
        -> Result<(), CommandError>
    {
        let volume = match args.single::<f32>() {
            Ok(volume) => volume,
            Err(_) => {
                check_msg(msg.channel_id.say("Volume must be a valid float between 0.0 and 1.0"));
                return Ok(());
            }
        };

        if volume < 0.0 || volume > 1.0 {
            check_msg(msg.channel_id.say("Volume must be between 0.0 and 1.0"));
            return Ok(());
        }

        let guild_id = match msg.guild_id {
            Some(guild_id) => guild_id,
            None => {
                check_msg(msg.channel_id.say("Groups and DMs not supported"));
                return Ok(());
            }
        };

        ctx.data.lock().get_mut::<VoiceGuilds>()
            .expect("Expected VoiceGuilds in ShareMap")
            .entry(guild_id).or_default()
            .set_volume(volume);

        check_msg(msg.channel_id.say(format!("Volume set to {}", volume)));

        Ok(())
    }
}

impl Command for Stop {
    fn options(&self) -> Arc<CommandOptions> {
        Arc::new(CommandOptions {
            help_available: true,
            desc: Some(String::from("Stop all clips currently being played by the bot")),
            ..Default::default()
        })
    }

    fn execute(&self, ctx: &mut Context, msg: &Message, _: Args)
        -> Result<(), CommandError>
    {
        let guild_id = match msg.guild_id {
            Some(guild_id) => guild_id,
            None => {
                check_msg(msg.channel_id.say("Groups and DMs not supported"));
                return Ok(());
            }
        };

        ctx.data.lock().get_mut::<VoiceGuilds>()
            .expect("Expected VoiceGuilds in ShareMap")
            .entry(guild_id).or_default()
            .clear_audios();

        Ok(())
    }
}

impl Command for Intro {
    fn options(&self) -> Arc<CommandOptions> {
        Arc::new(CommandOptions {
            help_available: true,
            desc: Some(String::from("Set the clip to be played when you enter the channel containing the bot")),
            usage: Some(String::from("<clip>")),
            example: Some(String::from("bnw/angels")),
            ..Default::default()
        })
    }

    fn execute(&self, ctx: &mut Context, msg: &Message, args: Args)
        -> Result<(), CommandError>
    {
        if args.len() != 1 {
            check_msg(msg.channel_id.say("Expected exactly one clip"));
            return Ok(());
        }

        let clip_str = args.current().unwrap();
        match get_clip(clip_str) {
            Some(_) => (),
            None => {
                check_msg(msg.channel_id.say("Invalid clip"));
                return Ok(());
            }
        }

        let user_id = msg.author.id;

        let mut data_lock = ctx.data.lock();
        let config = data_lock.get_mut::<ConfigResource>()
            .expect("Expected ConfigResource in ShareMap");

        config.intros.insert(user_id, clip_str.to_string());

        { use configuration::Result::*;
            match write_config(Path::new("config.json"), config) {
                Ok(()) => (),
                JsonError(reason) => eprintln!("Error writing config file: {:?}", reason),
                IoError(reason) => eprintln!("Error writing config file: {:?}", reason),
            }
        }

        check_msg(msg.channel_id.say("Set new intro"));
        Ok(())
    }
}

impl Command for BotIntro {
    fn options(&self) -> Arc<CommandOptions> {
        Arc::new(CommandOptions {
            help_available: true,
            desc: Some(String::from("Set the clip to be played when you enter the channel containing the bot")),
            usage: Some(String::from("<clip>")),
            example: Some(String::from("bnw/angels")),
            ..Default::default()
        })
    }

    fn execute(&self, ctx: &mut Context, msg: &Message, args: Args)
        -> Result<(), CommandError>
    {
        if args.len() != 1 {
            check_msg(msg.channel_id.say("Expected exactly one clip"));
            return Ok(());
        }

        let guild_id = match msg.guild_id {
            Some(guild_id) => guild_id,
            None => {
                check_msg(msg.channel_id.say("Groups and DMs not supported"));
                return Ok(());
            }
        };

        let clip_str = args.current().unwrap();
        match get_clip(clip_str) {
            Some(_) => (),
            None => {
                check_msg(msg.channel_id.say("Invalid clip"));
                return Ok(());
            }
        }

        let mut data_lock = ctx.data.lock();
        let config = data_lock.get_mut::<ConfigResource>()
            .expect("Expected ConfigResource in ShareMap");

        config.guilds
            .entry(guild_id)
            .or_default()
            .bot_intro = Some(clip_str.to_string());

        { use configuration::Result::*;
            match write_config(Path::new("config.json"), config) {
                Ok(()) => (),
                JsonError(reason) => eprintln!("Error writing config file: {:?}", reason),
                IoError(reason) => eprintln!("Error writing config file: {:?}", reason),
            }
        }

        check_msg(msg.channel_id.say("Set new intro"));
        Ok(())
    }

}

impl Command for Outro {
    fn options(&self) -> Arc<CommandOptions> {
        Arc::new(CommandOptions {
            help_available: true,
            desc: Some(String::from("Set the clip to be played when you exit the channel containing the bot")),
            usage: Some(String::from("<clip>")),
            example: Some(String::from("bnw/death")),
            ..Default::default()
        })
    }

    fn execute(&self, ctx: &mut Context, msg: &Message, args: Args)
        -> Result<(), CommandError>
    {
        if args.len() != 1 {
            check_msg(msg.channel_id.say("Expected exactly one clip"));
            return Ok(());
        }

        let clip_str = args.current().unwrap();
        match get_clip(clip_str) {
            Some(_) => (),
            None => {
                check_msg(msg.channel_id.say("Invalid clip"));
                return Ok(());
            }
        };

        let user_id = msg.author.id;

        let mut data_lock = ctx.data.lock();
        let config = data_lock.get_mut::<ConfigResource>()
            .expect("Expected ConfigResource in ShareMap");

        config.outros.insert(user_id, clip_str.to_string());

        { use configuration::Result::*;
            match write_config(Path::new("config.json"), config) {
                Ok(()) => (),
                JsonError(reason) => eprintln!("Error writing config file: {:?}", reason),
                IoError(reason) => eprintln!("Error writing config file: {:?}", reason),
            }
        }

        check_msg(msg.channel_id.say("Set new outro"));
        Ok(())
    }
}

impl Command for List {
    fn options(&self) -> Arc<CommandOptions> {
        Arc::new(CommandOptions {
            help_available: true,
            desc: Some(String::from("List all the sections and/or clips available in the section")),
            usage: Some(String::from("[section]")),
            example: Some(String::from("bnw")),
            ..Default::default()
        })
    }

    fn execute(&self, _: &mut Context, msg: &Message, args: Args)
        -> Result<(), CommandError>
    {
        if args.len() > 1 {
            check_msg(msg.channel_id.say("Expected at most one path to be specified"));
            return Ok(());
        }

        let dir = clip_path().join(Path::new(match args.len() {
            0 => "",
            1 => args.current().unwrap(),
            _ => {
                eprintln!("Unexpected number of arguments");
                return Ok(());
            }
        }));

        let dir = match dir.canonicalize() {
            Ok(dir) => dir,
            Err(_reason) => {
                check_msg(msg.channel_id.say("Invalid directory"));
                return Ok(());
            }
        };

        if !sandboxed(&clip_path(), &dir) {
            check_msg(msg.channel_id.say("Invalid directory"));
            return Ok(());
        }

        match read_dir(dir) {
            Err(reason) => {
                eprintln!("Unable to read directory: {:?}", reason);
                check_msg(msg.channel_id.say("Invalid directory"));
                return Ok(());
            }
            Ok(dir_iter) => {
                let message = dir_iter
                    .filter_map(|e| e.ok())
                    .map(|e| (e.path().file_stem().and_then(|f| f.to_str()).map(|f| f.to_owned()), e.file_type()))
                    .filter(|(f, t)| f.is_some() && t.is_ok())
                    .map(|(f, t)| (f.unwrap(), t.unwrap()))
                    .sorted_by(|(f0, t0), (f1, t1)| (!t0.is_dir(), f0.to_lowercase()).cmp(&(!t1.is_dir(), f1.to_lowercase())))
                    .map(|(f, t)| format!("{: <20}", f + if t.is_dir() { "/" } else { "" }))
                    .chunks(3)
                    .into_iter()
                    .map(|chunk| chunk.fold("".to_owned(), |acc, s| acc + &s))
                    .fold("".to_owned(), |acc, s| acc + "\n" + &s);

                check_msg(msg.channel_id.say(
                    "```\n".to_owned()
                    + &message
                    + "\n```"
                ));
            }
        }

        return Ok(());
    }
}

