use actix::prelude::*;

use crate::actors::{game_server::GameServer, player::Player};

/// A message to update duck state sent to `GameServer` actor
///
/// `GameServer` actor state updates duck with `Update` message

#[derive(Message)]
#[rtype(result = "()")]
pub struct Update {
    pub id: u32,
    pub duck: crate::duck::Duck,
}

impl Handler<Update> for GameServer {
    type Result = ();

    fn handle(&mut self, msg: Update, _: &mut Self::Context) -> Self::Result {
        if let Some(duck) = self.ducks.get_mut(&msg.id) {
            duck.x = msg.duck.x;
            duck.y = msg.duck.y;
            duck.z = msg.duck.z;
            duck.rotation_radians = msg.duck.rotation_radians;
        }
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct CastUpdateGame {
    pub update_data: Vec<u8>,
}

impl Handler<CastUpdateGame> for Player {
    type Result = ();

    fn handle(&mut self, message: CastUpdateGame, context: &mut Self::Context) -> Self::Result {
        context.binary(message.update_data);
    }
}
