use actix::prelude::*;

use crate::actors::GameServer;

/// TODO message not implemented yet
#[derive(Message)]
#[rtype("()")]
pub struct VoteStartGame {}

impl Handler<VoteStartGame> for GameServer {
    type Result = ();

    fn handle(&mut self, message: VoteStartGame, context: &mut Self::Context) -> Self::Result {
        // TODO !!!
    }
}
