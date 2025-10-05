use actix::prelude::*;

use crate::actors::game_server::GameServer;

#[derive(Message)]
#[rtype(result = "()")]
pub struct JoinLobby {
    pub id: u32,
    pub name: String,
}

impl Handler<JoinLobby> for GameServer {
    type Result = ();

    fn handle(&mut self, _msg: JoinLobby, _: &mut Context<Self>) {
        panic!("NOT IMPLEMENTED YET");
    }
}
