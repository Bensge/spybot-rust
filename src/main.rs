use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use tokio::time::sleep;
use ts3::client::ServerNotifyRegister;
use ts3::event::{ClientMoved, TextMessage};
use ts3::{
    client::{async_trait, TextMessageTarget},
    event::{ClientEnterView, EventHandler},
    Client, RawResp,
};
use ts3::{Decode, Error};

use crate::listener::Listener;
use crate::user::User;

mod listener;
mod user;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Create a new client
    let client = Client::new("teeess.bensge.com:10011").await?;

    // Connect to virtualserver 1
    client.use_sid(1).await?;

    // Use whoami to fetch info about the query client
    let data = client.whoami().await?;

    println!("whoami = {:?}", data);

    client.login(&"tsmonitor", &"HRFVuApZ").await?;

    client.use_port(9987).await?;

    configure_unique_nickname(&client).await;

    let users = Arc::new(Mutex::new(HashMap::new()));

    let users_ref = users.clone();
    let mut usr = users_ref.lock().unwrap();
    for (idx, user) in client.clientlist().await?.iter().enumerate() {
        println!("{} client {:?}", idx, user);

        let u = User {
            clid: user.clid,
            unique_id: user.client_unique_identifier.clone(),
            nickname: user.client_nickname.clone(),
            is_query_user: user.client_type == 1,
        };
        usr.insert(u.clid, u);

        // send_message(
        //     &client,
        //     user.clid,
        //     "Check out todays sponsor of BensgeTS on http://amzn.to/2CUD262",
        // )
        //     .await;
        // println!("sending message");
        sleep(Duration::from_millis(2000)).await;
    }
    drop(usr);

    // Assign a new event handler.
    let handler = Listener::new(users.clone());
    client.set_event_handler(handler);

    client
        .servernotifyregister(ServerNotifyRegister::Server)
        .await?;
    client
        .servernotifyregister(ServerNotifyRegister::Channel(0))
        .await?;
    client
        .servernotifyregister(ServerNotifyRegister::TextPrivate)
        .await?;

    tokio::signal::ctrl_c().await?;

    println!("logging out...");
    client.logout().await?;
    println!("disconnecting...");
    client.quit().await?;

    Ok(())
}

async fn configure_unique_nickname(client: &Client) {
    let mut name_postfix = 0;
    loop {
        let name = format!(
            "spybot-oxidized{}",
            if name_postfix == 0 {
                "".to_string()
            } else {
                name_postfix.to_string()
            }
        );
        println!("Trying name {name}");
        let res: ts3::client::Result<RawResp> = client
            .send(format!("clientupdate client_nickname={}", name))
            .await;

        if res.is_err() {
            if let Err(Error::TS3 {
                id: 513, /* nickname already in use */
                msg: _,
            }) = res
            {
                name_postfix += 1;
                sleep(Duration::from_millis(200)).await;
                continue;
            }
        }
        break;
    }
}

async fn send_message(client: &Client, client_id: usize, text: &str) {
    // Send a private message to the client using "sendtextmessage".
    let send_message_result = client
        .sendtextmessage(TextMessageTarget::Client(client_id), &text)
        .await;

    if let Err(error) = send_message_result {
        println!("Error sending text message {}", error)
    }
}

#[cfg(test)]
mod test {}
