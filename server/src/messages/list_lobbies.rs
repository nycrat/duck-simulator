use actix::prelude::*;

use crate::{actors::game_server::GameServer, messages};

#[derive(Message)]
#[rtype(result = "Vec<String>")]
pub struct ListLobbies;

impl Handler<ListLobbies> for GameServer {
    type Result = MessageResult<messages::ListLobbies>;

    fn handle(&mut self, _: messages::ListLobbies, _: &mut Context<Self>) -> Self::Result {
        let lobbies = self.lobbies.keys().map(|lobby| lobby.to_owned()).collect();

        MessageResult(lobbies)
    }
}
