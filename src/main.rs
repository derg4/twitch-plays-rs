use dotenv::dotenv;
use dotenv_codegen::dotenv;
use futures::prelude::*;
use tokio::sync::mpsc;

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
    let (mut cm_tx, mut cm_rx) = mpsc::channel(100);
    tokio::spawn(async move {
        CommandManager::process_forever(cm_rx).await;
    });
    /*tokio::spawn(async move {
        while let Some(message) = cm_rx.recv().await {
            println!("Received message: {:?}", message);
        }
    });*/

    let (mut incoming_messages, client) =
        TwitchIRCClient::<TCPTransport, StaticLoginCredentials>::new(twitch_config);

    // first thing you should do: start consuming incoming messages,
    // otherwise they will back up.
    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.next().await {
            cm_tx.send(message).await.unwrap();
            //command_manager.process_message(&message).await
            //tokio::spawn(async {
             //   command_manager.process_message(&message).await
            //});
        }
    });

    // join a channel
    client.join("derg_bot".to_owned());

    // keep the tokio executor alive.
    // If you return instead of waiting the background task will exit.
    join_handle.await.unwrap();
}
