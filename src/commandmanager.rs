use std::time;
use enigo::*;
use twitch_irc::message::*;
use tokio::sync::mpsc;

const test_commands: [Command; 1] = [Command::key_cmd("!right", Key::Layout('d'), 5000)];

pub struct CommandManager<'a> {
    enigo: Enigo,
    commands: Vec<Command<'a>>,
}
impl<'a> CommandManager<'a> {
    pub async fn process_forever(mut rx: mpsc::Receiver<ServerMessage>) {
        let mut cm = CommandManager {
            enigo: Enigo::new(),
            commands: Vec::from(test_commands),
        };

        while let Some(message) = rx.recv().await {
            tokio::spawn(async {
                //command_manager.process_message(&message).await
                cm.process_message(&message).await;
            });
        }
        println!("Exited process_forever");
    }

    pub async fn process_message(&mut self, message: &ServerMessage) {
        if let ServerMessage::Privmsg(privmsg) = message {
            println!("Got a message! {:?}", message);
            for command in &self.commands {
                if command.check_message(privmsg, &mut self.enigo).await {
                    break;
                }
            }
        }
    }
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

    async fn check_message(&self, privmsg: &PrivmsgMessage, enigo: &mut Enigo) -> bool {
        if privmsg.sender.login == "dgjfe" && privmsg.message_text.starts_with(&self.trigger) {
            /*
             * Consider tokio's Mutex to make mut data safe to access when asyncly called
             * Consider creating a Waker and poll the tokio Delay manually...
             * See std::task::Waker::From<Arc<Wake>>
             */
            self.do_action(enigo).await;
            true
        }
        else {
            false
        }
    }

    async fn do_action(&self, enigo: &mut Enigo) {
        println!("Doing action {:?}", self.action);
        self.start_action(enigo);

        tokio::time::delay_for(self.dur).await;

        //thread::sleep(self.dur);
        println!("Ending action {:?}", self.action);
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

#[derive(Debug)]
pub enum Action {
    KeyAction(Key),
    SomethingElse,
}
