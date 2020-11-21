use std::{thread, time};
use enigo::*;
use twitch_irc::message::*;

pub struct CommandManager<'a> {
    commands: Vec<Command<'a>>,
    enigo: Enigo,
}

pub struct Command<'a>{
    pub trigger: &'a str,
    pub action: Action,
    pub dur: time::Duration,
}
impl<'a> Command<'a> {
    const fn new(trigger: &'a str, action: Action, dur_ms: u64) -> Command<'a> {
        Command {
            trigger: trigger,
            action: action,
            dur: time::Duration::from_millis(dur_ms),
        }
    }

    const fn key_cmd(trigger: &'a str, key: Key, dur_ms: u64) -> Command<'a> {
        Command::new(trigger, Action::KeyAction(key), dur_ms)
    }

    fn check_message(&self, privmsg: &PrivmsgMessage, enigo: &mut Enigo) -> bool {
        if privmsg.sender.login == "dgjfe" && privmsg.message_text.starts_with(&self.trigger) {
            self.do_action(enigo);
            true
        }
        else {
            false
        }
    }

    //TODO learn tokio
    fn do_action(&self, enigo: &mut Enigo) {
        self.start_action(enigo);
        thread::sleep(self.dur);
        self.end_action(enigo);
    }

    fn start_action(&self, enigo: &mut Enigo) {
        match self.action {
            Action::KeyAction(key) => enigo.key_down(key),
            _ => (),
        }
    }

    fn end_action(&self, enigo: &mut Enigo) {
        match self.action {
            Action::KeyAction(key) => enigo.key_up(key),
            _ => (),
        }
    }
}

pub enum Action {
    KeyAction(Key),
    SomethingElse,
}

const test_commands: [Command; 1] = [Command::key_cmd("!right", Key::Layout('d'), 10000)];

impl<'a> CommandManager<'a> {
    pub fn new() -> CommandManager<'static> {
        CommandManager {
            enigo: Enigo::new(),
            commands: Vec::from(test_commands),
        }
    }

    pub fn process_message(&mut self, message: &ServerMessage) {
        if let ServerMessage::Privmsg(privmsg) = message {
            for command in &self.commands {
                if command.check_message(privmsg, &mut self.enigo) {
                    break;
                }
            }
        }
    }
}
