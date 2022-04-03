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

use crate::config::Config;
use crate::ts_interface::{TSInterface};
use crate::user::User;
use config_file::FromConfigFile;
use db::DB;

mod config;
mod db;
mod db_scheme;
mod ts_interface;
mod user;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let c = Config::new();

    let mut database = DB::connect(&c)?;
    database.get_total_users();

    // Create a new client
    let client = Client::new("teeess.bensge.com:10011").await?;

    // Connect to virtualserver 1
    client.use_sid(1).await?;

    // Use whoami to fetch info about the query client
    let data = client.whoami().await?;

    println!("whoami = {:?}", data);


    client.login(&c.ts_name, &c.ts_password).await?;
    client.use_port(c.ts_port).await?;

    configure_unique_nickname(&client).await;

    let users = Arc::new(Mutex::new(HashMap::new()));



    // Assign a new event handler.
    let mut interface = TSInterface::new(users.clone());
    interface.init(&client).await?;
    client.set_event_handler(interface);



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
