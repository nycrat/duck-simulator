use actix::prelude::*;

use crate::actors::player::Player;

#[derive(Message)]
#[rtype(result = "()")]
pub struct CastEndGame {}

impl Handler<CastEndGame> for Player {
    type Result = ();

    fn handle(&mut self, _: CastEndGame, context: &mut Self::Context) -> Self::Result {
        context.text("cast:end_game");
    }
}
