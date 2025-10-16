use actix::prelude::*;

use crate::actors::Player;

/// A message to `Player` actor to broadcast game has ended
#[derive(Message)]
#[rtype("()")]
pub struct CastEndGame {}

impl Handler<CastEndGame> for Player {
    type Result = ();

    fn handle(&mut self, _: CastEndGame, context: &mut Self::Context) -> Self::Result {
        context.text("cast:end_game");
    }
}
