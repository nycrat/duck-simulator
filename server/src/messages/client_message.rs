use actix::prelude::*;

use crate::actors::game_server::GameServer;

#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    pub id: u32,
    pub message: String,
    pub lobby: String,
}

impl Handler<ClientMessage> for GameServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>) {
        self.send_message_to_lobby(&msg.lobby, msg.message.as_str(), msg.id);
    }
}
