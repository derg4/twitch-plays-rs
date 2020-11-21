use std::{thread, time};
use enigo::*;
use twitch_irc::message::*;

pub struct CommandManager {
    enigo: Enigo,
}

/*pub enum Command {
    KeyCommand,
    MouseCommand
}*/

// KeyCommand: key, dur
//

impl CommandManager {
    pub fn new() -> CommandManager {
        CommandManager {
            enigo: Enigo::new(),
        }
    }

    pub fn process_message(&mut self, message: &ServerMessage) {
        if let ServerMessage::Privmsg(privmsg) = message {
            if privmsg.sender.login == "dgjfe" && privmsg.message_text == "!right" {
                thread::sleep(time::Duration::from_millis(2000));
                self.enigo.key_down(Key::Layout('d'));
                thread::sleep(time::Duration::from_millis(2000));
                self.enigo.key_up(Key::Layout('d'));

                println!("Privmsg {:?}", privmsg);
            }
        }
    }
}
