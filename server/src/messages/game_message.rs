use actix::prelude::*;

use crate::actors::player::Player;

// TODO refactor game message as enum cause it makes more sense
#[derive(Message)]
#[rtype(result = "()")]
pub struct GameMessage(pub Option<String>, pub Option<Vec<u8>>);

impl Handler<GameMessage> for Player {
    type Result = ();

    fn handle(&mut self, message: GameMessage, context: &mut Self::Context) {
        if message.0.is_some() {
            context.text(message.0.unwrap());
        }
        if message.1.is_some() {
            context.binary(message.1.unwrap());
        }
    }
}
