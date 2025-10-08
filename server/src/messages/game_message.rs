use actix::prelude::*;

use crate::actors::player::Player;

/// A message sent to `Player` actor
///
/// Used by `GameServer` actor to send message to `Player` actor

// TODO refactor game message as enum cause it makes more sense
#[derive(Message)]
#[rtype(result = "()")]
pub struct GameMessage(pub Option<String>, pub Option<Vec<u8>>);

impl Handler<GameMessage> for Player {
    type Result = ();

    fn handle(&mut self, message: GameMessage, player_context: &mut Self::Context) {
        if message.0.is_some() {
            player_context.text(message.0.unwrap());
        }
        if message.1.is_some() {
            player_context.binary(message.1.unwrap());
        }
    }
}
