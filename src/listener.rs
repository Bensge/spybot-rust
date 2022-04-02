use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::user::User;
use ts3::event::{ClientLeftView, ClientMoved, TextMessage};
use ts3::{
    client::{async_trait, TextMessageTarget},
    event::{ClientEnterView, EventHandler},
    Client, RawResp,
};

pub(crate) struct Listener {
    /// map from clid -> User
    users: Arc<Mutex<HashMap<u64, User>>>,
}

impl Listener {
    pub fn new(users: Arc<Mutex<HashMap<u64, User>>>) -> Self {
        Self {
            users: users.clone(),
        }
    }
}

#[async_trait]
impl EventHandler for Listener {
    async fn cliententerview(&self, _client: Client, event: ClientEnterView) {
        println!(
            "Client {} aka {} joined!",
            &event.clid, &event.client_nickname
        );

        let users_ref = self.users.clone();
        let mut users = users_ref.lock().unwrap();
        match users.get(&event.clid) {
            Some(_user) => {
                println!("Duplicate user {} {}", event.clid, event.client_nickname);
            }

            None => {
                let u = User {
                    clid: event.clid,
                    unique_id: event.client_unique_identifier,
                    nickname: event.client_nickname,
                    is_query_user: event.client_type == 1,
                };

                users.insert(u.clid, u);
            }
        }

        // send_message(
        //     &client,
        //     event.clid,
        //     "Hello World from Spybot-oxidized!",
        // )
        // .await;
    }

    async fn clientleftview(&self, _client: Client, event: ClientLeftView) {
        let users_ref = self.users.clone();
        let mut users = users_ref.lock().unwrap();

        if let Some(user) = users.get(&event.clid) {
            println!("User left: {}", user.nickname);

            users.remove(&event.clid);
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
