use ts3::client::ServerNotifyRegister;
use ts3::event::{ClientMoved, TextMessage};
use ts3::{
    client::{async_trait, TextMessageTarget},
    event::{ClientEnterView, EventHandler},
    Client, RawResp,
};
use ts3::{Decode, Error};

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
                continue;
            }
        }
        break;
    }

    // Assign a new event handler.
    client.set_event_handler(Handler);

    client
        .servernotifyregister(ServerNotifyRegister::Server)
        .await?;
    client
        .servernotifyregister(ServerNotifyRegister::Channel(0))
        .await?;
    client
        .servernotifyregister(ServerNotifyRegister::TextPrivate)
        .await?;

    for (idx, user) in client.clientlist().await?.iter().enumerate() {
        println!("{} client {:?}", idx, user);
    }

    tokio::signal::ctrl_c().await?;

    println!("logging out...");
    client.logout().await?;
    println!("disconnecting...");
    client.quit().await?;

    Ok(())
}

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn cliententerview(&self, client: Client, event: ClientEnterView) {
        println!(
            "Client {} aka {} joined!",
            event.clid, event.client_nickname
        );

        // Send a private message to the client using "sendtextmessage".
        let send_message_result = client
            .sendtextmessage(
                TextMessageTarget::Client(event.clid as usize),
                "Hello World from Spybot-oxidized!",
            )
            .await;

        if let Err(error) = send_message_result {
            println!("Error sending text message {}", error)
        }
    }

    async fn clientmoved(&self, _client: Client, event: ClientMoved) {
        let user_id = event.clid;
        let channel_id = event.ctid; // channel-to-id as opposed to channel-from-id
        println!("Client {user_id} moved to channel {channel_id}!");
    }

    async fn textmessage(&self, _client: Client, event: TextMessage) {
        let message = event.msg;
        let sender_id = event.invokerid;
        let invoker_name = event.invokername;
        println!("Got text message {message} from {sender_id} aka {invoker_name}");
    }
}

#[cfg(test)]
mod test {
}
