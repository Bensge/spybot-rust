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
    use ts3::event::ClientEnterView;
    use ts3::Decode;

    #[test]
    fn test_cev() {
        let raw = "
cfid=0
ctid=1
reasonid=0
clid=1235
client_unique_identifier=T2FvlJL+js6a=
client_nickname=testuser
client_input_muted=0
client_output_muted=0
client_outputonly_muted=0
client_input_hardware=0
client_output_hardware=0
client_meta_data
client_is_recording=0
client_database_id=987
client_channel_group_id=8
client_servergroups=6,11
client_away=0
client_away_message
client_type=0
client_flag_avatar
client_talk_power=75
client_talk_request=0
client_talk_request_msg
client_description
client_is_talker=0
client_is_priority_speaker=0
client_unread_messages=0
client_nickname_phonetic
client_needed_serverquery_view_power=75
client_icon_id=0
client_is_channel_commander=0
client_country=DE
client_channel_group_inherited_channel_id=1
client_badges
client_myteamspeak_id
client_integrations
client_myteamspeak_avatar
client_signed_badges
client_estimated_location
client_needed_serverquey_view_power=75
client_output_hardwarer=0
";
        let buf = raw.replace("\n", " ");

        ClientEnterView::decode(buf.as_bytes()).unwrap();
    }

    #[test]
    fn test_vec_decode() {
        let buf = b"86,5";
        assert_eq!(Vec::<usize>::decode(buf).unwrap(), vec![86, 5]);
    }
}
