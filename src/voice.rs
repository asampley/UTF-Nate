use serenity::framework::standard::{Args, CommandResult};
use serenity::framework::standard::macros::{command, group};
use serenity::model::channel::Message;
use serenity::client::Context;
use serenity::voice::AudioSource;
use serenity::voice;

use std::fs::read_dir;
use std::path::{Path, PathBuf};

use itertools::Itertools;

use crate::data::{VoiceManager, VoiceGuilds, ConfigResource};
use crate::configuration;
use crate::configuration::write_config;
use crate::util::*;

#[group("voice")]
#[description("Commands to move the bot to voice channels, play clips, and set intro/outro clips for each user.")]
#[commands(summon, banish, play, volume, stop, intro, outro, playlist, introbot)]
pub struct Voice;

fn clip_path() -> PathBuf {
    return Path::new("./resources/clips").canonicalize().unwrap();
}

pub async fn audio_source(loc: &str) -> serenity::Result<Box<dyn AudioSource>> {
    if loc.starts_with("http") {
        return voice::ytdl(&loc).await
    } else {
        match get_clip(&loc) {
            Some(clip) => voice::ffmpeg(&clip).await,
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
    sandboxed_exists(&clip_path(), &path)
}

#[command]
#[help_available]
#[description("Summon the bot to the voice channel the user is currently in")]
pub async fn summon(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = match msg.guild(&ctx.cache).await {
        Some(guild) => guild,
        None => {
            check_msg(msg.channel_id.say(&ctx.http, "Groups and DMs not supported").await);
            return Ok(());
        },
    };

    let guild_id = guild.id;

    let channel_id = guild
        .voice_states.get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            check_msg(msg.reply(&ctx, "Not in a voice channel").await);
            return Ok(());
        }
    };

    let manager_lock = ctx.data.read().await.get::<VoiceManager>().cloned().expect("Expected VoiceManager in ShareMap");
    let mut manager = manager_lock.lock().await;

    if manager.join(guild_id, connect_to).is_none() {
        check_msg(msg.channel_id.say(&ctx.http, "Error joining the channel").await);
    }

    Ok(())
}

#[command]
#[help_available]
#[description("Remove the bot from the voice channel it is in")]
pub async fn banish(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = match msg.guild(&ctx.cache).await {
        Some(guild) => guild,
        None => {
            check_msg(msg.channel_id.say(&ctx.http, "Groups and DMs not supported").await);
            return Ok(());
        }
    };

    let guild_id = guild.id;

    let manager_lock = ctx.data.read().await.get::<VoiceManager>().cloned().expect("Expected VoiceManager in ShareMap");
    let mut manager = manager_lock.lock().await;
    let has_handler = manager.get(guild_id).is_some();

    if has_handler {
        manager.remove(guild_id);
    } else {
        check_msg(msg.reply(&ctx, "Not in a voice channel").await);
    }

    Ok(())
}

#[command]
#[help_available]
#[description("Play the specified clip")]
#[num_args(1)]
#[usage("<clip>")]
#[example("bnw/needoffspring")]
pub async fn play(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let loc = match args.single::<String>() {
        Ok(loc) => loc,
        Err(_) => {
            check_msg(msg.channel_id.say(&ctx.http, "Must provide a source").await);
            return Ok(())
        },
    };

    let guild = match msg.guild(&ctx.cache).await {
        Some(guild) => guild,
        None => {
            check_msg(msg.channel_id.say(&ctx.http, "Groups and DMs not supported").await);
            return Ok(());
        }
    };

    let guild_id = guild.id;

    {
        let mut data_lock = ctx.data.write().await;

        let manager_lock = data_lock.get::<VoiceManager>().cloned().expect("Expected VoiceManager in ShareMap");
        let mut manager = manager_lock.lock().await;

        let voice_guild_arc = data_lock.get_mut::<VoiceGuilds>()
            .cloned()
            .expect("Expected VoiceGuilds in ShareMap")
            .write().await
            .entry(guild_id)
            .or_default()
            .clone();

        let mut voice_guild = voice_guild_arc.write().await;

        if let Some(handler) = manager.get_mut(guild_id) {
            let source = audio_source(&loc).await;

            match source {
                Ok(source) => {
                    voice_guild.add_audio(handler.play_returning(source)).await;
                },
                Err(reason) => {
                    eprintln!("Error trying to play clip: {:?}", reason);
                    check_msg(msg.channel_id.say(&ctx.http, "Invalid clip").await);
                }
            }
        } else {
            check_msg(msg.channel_id.say(&ctx.http, "Not in a voice channel").await);
        }
    }

    Ok(())
}

#[command]
#[help_available]
#[description("Stop all clips currently being played by the bot")]
#[num_args(1)]
#[usage("<volume>")]
#[example("0.5")]
pub async fn volume(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let volume = match args.single::<f32>() {
        Ok(volume) => volume,
        Err(_) => {
            check_msg(msg.channel_id.say(&ctx.http, "Volume must be a valid float between 0.0 and 1.0").await);
            return Ok(());
        }
    };

    if volume < 0.0 || volume > 1.0 {
        check_msg(msg.channel_id.say(&ctx.http, "Volume must be between 0.0 and 1.0").await);
        return Ok(());
    }

    let guild_id = match msg.guild_id {
        Some(guild_id) => guild_id,
        None => {
            check_msg(msg.channel_id.say(&ctx.http, "Groups and DMs not supported").await);
            return Ok(());
        }
    };

    ctx.data.write().await.get_mut::<VoiceGuilds>()
        .expect("Expected VoiceGuilds in ShareMap")
        .write().await
        .entry(guild_id).or_default().clone().write().await
        .set_volume(volume).await;

    check_msg(msg.channel_id.say(&ctx.http, format!("Volume set to {}", volume)).await);

    Ok(())
}

#[command]
#[help_available]
#[description("Stop all clips currently being played by the bot")]
pub async fn stop(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = match msg.guild_id {
        Some(guild_id) => guild_id,
        None => {
            check_msg(msg.channel_id.say(&ctx.http, "Groups and DMs not supported").await);
            return Ok(());
        }
    };

    ctx.data.write().await.get_mut::<VoiceGuilds>()
        .expect("Expected VoiceGuilds in ShareMap")
        .write().await
        .entry(guild_id).or_default()
        .clone().write().await
        .clear_audios().await;

    Ok(())
}

#[command]
#[help_available]
#[description("Set the clip to be played when you enter the channel containing the bot")]
#[num_args(1)]
#[usage("<clip>")]
#[example("bnw/angels")]
pub async fn intro(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if args.len() != 1 {
        check_msg(msg.channel_id.say(&ctx.http, "Expected exactly one clip").await);
        return Ok(());
    }

    let clip_str = args.current().unwrap();
    match get_clip(clip_str) {
        Some(_) => (),
        None => {
            check_msg(msg.channel_id.say(&ctx.http, "Invalid clip").await);
            return Ok(());
        }
    }

    let user_id = msg.author.id;

    let mut data_lock = ctx.data.write().await;
    let config_arc = data_lock.get_mut::<ConfigResource>()
        .expect("Expected ConfigResource in ShareMap")
        .clone();

    let mut config = config_arc.write().await;

    config.intros.insert(user_id, clip_str.to_string());

    { use configuration::Result::*;
        match write_config(Path::new("config.json"), &*config) {
            Ok(()) => (),
            JsonError(reason) => eprintln!("Error writing config file: {:?}", reason),
            IoError(reason) => eprintln!("Error writing config file: {:?}", reason),
        }
    }

    check_msg(msg.channel_id.say(&ctx.http, "Set new intro").await);
    Ok(())
}

#[command]
#[help_available]
#[description("Set the clip to be played when you enter the channel containing the bot")]
#[num_args(1)]
#[usage("<clip>")]
#[example("bnw/angels")]
pub async fn introbot(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if args.len() != 1 {
        check_msg(msg.channel_id.say(&ctx.http, "Expected exactly one clip").await);
        return Ok(());
    }

    let guild_id = match msg.guild_id {
        Some(guild_id) => guild_id,
        None => {
            check_msg(msg.channel_id.say(&ctx.http, "Groups and DMs not supported").await);
            return Ok(());
        }
    };

    let clip_str = args.current().unwrap();
    match get_clip(clip_str) {
        Some(_) => (),
        None => {
            check_msg(msg.channel_id.say(&ctx.http, "Invalid clip").await);
            return Ok(());
        }
    }

    let mut data_lock = ctx.data.write().await;
    let config_arc = data_lock.get_mut::<ConfigResource>()
        .expect("Expected ConfigResource in ShareMap")
        .clone();

    let mut config = config_arc.write().await;

    config.guilds
        .entry(guild_id)
        .or_default()
        .bot_intro = Some(clip_str.to_string());

    { use configuration::Result::*;
        match write_config(Path::new("config.json"), &*config) {
            Ok(()) => (),
            JsonError(reason) => eprintln!("Error writing config file: {:?}", reason),
            IoError(reason) => eprintln!("Error writing config file: {:?}", reason),
        }
    }

    check_msg(msg.channel_id.say(&ctx.http, "Set new intro").await);
    Ok(())
}

#[command]
#[help_available]
#[description("Set the clip to be played when you exit the channel containing the bot")]
#[num_args(1)]
#[usage("<clip>")]
#[example("bnw/death")]
pub async fn outro(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if args.len() != 1 {
        check_msg(msg.channel_id.say(&ctx.http, "Expected exactly one clip").await);
        return Ok(());
    }

    let clip_str = args.current().unwrap();
    match get_clip(clip_str) {
        Some(_) => (),
        None => {
            check_msg(msg.channel_id.say(&ctx.http, "Invalid clip").await);
            return Ok(());
        }
    };

    let user_id = msg.author.id;

    let mut data_lock = ctx.data.write().await;
    let config_arc = data_lock.get_mut::<ConfigResource>()
        .expect("Expected ConfigResource in ShareMap")
        .clone();

    let mut config = config_arc.write().await;

    config.outros.insert(user_id, clip_str.to_string());

    { use configuration::Result::*;
        match write_config(Path::new("config.json"), &*config) {
            Ok(()) => (),
            JsonError(reason) => eprintln!("Error writing config file: {:?}", reason),
            IoError(reason) => eprintln!("Error writing config file: {:?}", reason),
        }
    }

    check_msg(msg.channel_id.say(&ctx.http, "Set new outro").await);
    Ok(())
}

#[command]
#[help_available]
#[description("List all the sections and/or clips available in the section")]
#[min_args(0)]
#[max_args(1)]
#[usage("[section]")]
#[example("bnw")]
pub async fn playlist(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if args.len() > 1 {
        check_msg(msg.channel_id.say(&ctx.http, "Expected at most one path to be specified").await);
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
            check_msg(msg.channel_id.say(&ctx.http, "Invalid directory").await);
            return Ok(());
        }
    };

    if !sandboxed_exists(&clip_path(), &dir) {
        check_msg(msg.channel_id.say(&ctx.http, "Invalid directory").await);
        return Ok(());
    }

    match read_dir(dir) {
        Err(reason) => {
            eprintln!("Unable to read directory: {:?}", reason);
            check_msg(msg.channel_id.say(&ctx.http, "Invalid directory").await);
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

            check_msg(msg.channel_id.say(&ctx.http, 
                "```\n".to_owned()
                + &message
                + "\n```"
            ).await);
        }
    }

    return Ok(());
}

