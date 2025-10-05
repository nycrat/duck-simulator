use actix::prelude::*;

use crate::actors::game_server::GameServer;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: u32,
}

impl Handler<Disconnect> for GameServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        let mut lobbies: Vec<String> = Vec::new();

        // remove address
        if self.clients.remove(&msg.id).is_some() || self.ducks.remove(&msg.id).is_some() {
            // remove session from all lobbies
            for (name, lobby) in &mut self.lobbies {
                if lobby.duck_map.remove(&msg.id).is_some() {
                    lobbies.push(name.to_owned());
                }
                lobby.spectator_ids.remove(&msg.id);
            }
        }

        // send message to other users
        for lobby in lobbies {
            self.send_message_to_lobby(&lobby, &format!("/disconnect\n{}", msg.id), 0);
        }
    }
}
