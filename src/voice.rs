use serenity::framework::standard::{Args, Command, CommandError};
use serenity::client::bridge::voice::ClientVoiceManager;
use serenity::model::channel::Message;
use serenity::client::Context;
use serenity::Result as SerenityResult;
use serenity::prelude::*;

use std::sync::Arc;

pub struct VoiceManager;

impl TypeMapKey for VoiceManager {
    type Value = Arc<Mutex<ClientVoiceManager>>;
}

pub struct Join;
pub struct Leave;

fn check_msg(result: SerenityResult<Message>) {
    if let Err(reason) = result {
        eprintln!("Error sending message: {:?}", reason);
    }
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
