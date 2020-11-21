use dotenv::dotenv;
use dotenv_codegen::dotenv;
use futures::prelude::*;
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::ClientConfig;
use twitch_irc::TCPTransport;
use twitch_irc::TwitchIRCClient;

mod commandmanager;
use commandmanager::CommandManager;

#[tokio::main]
pub async fn main() {
    dotenv().ok();
    /*let twitch_config = ClientConfig::new_simple(
        StaticLoginCredentials::new(dotenv!("BOT_USERNAME").to_owned(),
                                    Some(dotenv!("BOT_OAUTH_TOKEN").to_owned()))
    );*/
    let twitch_config = ClientConfig::default();
    let mut command_manager = CommandManager::new();

    let (mut incoming_messages, client) =
        TwitchIRCClient::<TCPTransport, StaticLoginCredentials>::new(twitch_config);

    // first thing you should do: start consuming incoming messages,
    // otherwise they will back up.
    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.next().await {
            //println!("Received message: {:?}", message);
            command_manager.process_message(&message);
        }
    });

    // join a channel
    client.join("dgjfe".to_owned());

    // keep the tokio executor alive.
    // If you return instead of waiting the background task will exit.
    join_handle.await.unwrap();
}
