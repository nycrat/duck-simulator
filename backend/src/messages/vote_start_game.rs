use actix::prelude::*;

use crate::actors::game_server::GameServer;

#[derive(Message)]
#[rtype(result = "()")]
pub struct VoteStartGame {}

impl Handler<VoteStartGame> for GameServer {
    type Result = MessageResult<VoteStartGame>;

    fn handle(&mut self, message: VoteStartGame, context: &mut Self::Context) -> Self::Result {
        // TODO !!!
        MessageResult(())
    }
}
