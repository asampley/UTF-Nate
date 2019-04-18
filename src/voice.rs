use serenity::framework::standard::{Args, Command, CommandError};
use serenity::model::channel::Message;
use serenity::client::Context;
use serenity::voice;
use serenity::Result as SerenityResult;
use serenity::voice::AudioSource;

use std::path::{Path, PathBuf};

use crate::data::{VoiceManager, VoiceGuilds, VoiceGuild};

fn clip_path() -> PathBuf {
    return Path::new("./resources/clips").canonicalize().unwrap();
}

pub struct Join;
pub struct Leave;
pub struct Play;
pub struct Volume;
pub struct Stop;

fn check_msg(result: SerenityResult<Message>) {
    if let Err(reason) = result {
        eprintln!("Error sending message: {:?}", reason);
    }
}

pub fn audio_source(loc: &str) -> serenity::Result<Box<dyn AudioSource>> {
    if loc.starts_with("http") {
        return voice::ytdl(&loc)
    } else {
        // create a new path, but ensure the path does not go above
        // the clip directory
        let clip_path = clip_path();
        let mut play_path = clip_path.join(&loc);
        for ext in &["mp3", "wav"] {
            play_path.set_extension(ext);
            match play_path.canonicalize() {
                Ok(play_path) => {
                    if play_path.ancestors().all(|a| a != clip_path) {
                        return Err(serenity::Error::Other("Attempt to play clip outside clip path"));
                    } else if !play_path.exists() {
                        // skip extension
                    } else {
                        return voice::ffmpeg(&play_path);
                    }
                },
                Err(_) => () // just move on to next extension
            }
        }
    };

    Err(serenity::Error::Other("Could not find source"))
}


impl Command for Join {
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
    fn execute(&self, ctx: &mut Context, msg: &Message, mut args: Args)
        -> Result<(), CommandError>
    {
        let volume = match args.single::<f32>() {
            Ok(volume) => volume,
            Err(_) => {
                check_msg(msg.channel_id.say("`Volume must be a valid float between 0.0 and 1.0`"));
                return Ok(());
            }
        };

        if volume < 0.0 || volume > 1.0 {
            check_msg(msg.channel_id.say("`Volume must be between 0.0 and 1.0`"));
            return Ok(());
        }

        let guild_id = match msg.guild_id {
            Some(guild_id) => guild_id,
            None => {
                check_msg(msg.channel_id.say("`Groups and DMs not supported`"));
                return Ok(());
            }
        };

        ctx.data.lock().get_mut::<VoiceGuilds>()
            .expect("Expected VoiceGuilds in ShareMap")
            .entry(guild_id).or_default()
            .set_volume(volume);

        check_msg(msg.channel_id.say(format!("`Volume set to {}`", volume)));

        Ok(())
    }
}

impl Command for Stop {
    fn execute(&self, ctx: &mut Context, msg: &Message, _: Args)
        -> Result<(), CommandError>
    {
        let guild_id = match msg.guild_id {
            Some(guild_id) => guild_id,
            None => {
                check_msg(msg.channel_id.say("`Groups and DMs not supported`"));
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
