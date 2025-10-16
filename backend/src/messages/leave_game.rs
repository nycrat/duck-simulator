use actix::prelude::*;

use crate::actors::{GameServer, Player};

/// A message to `GameServer` actor that a duck has left the game
#[derive(Message)]
#[rtype("()")]
pub struct LeaveGame {
    pub id: u32,
}

impl Handler<LeaveGame> for GameServer {
    type Result = ();

    fn handle(&mut self, message: LeaveGame, _: &mut Context<Self>) {
        log::info!("duck disconnected");
        if self.player_actors.remove(&message.id).is_some()
            || self.ducks.remove(&message.id).is_some()
        {
            self.player_actors.iter().for_each(|(_, actor)| {
                actor.do_send(CastLeaveGame { id: message.id });
            });
        }
    }
}

/// A message to `Player` actor to broadcast a duck has left the game
#[derive(Message)]
#[rtype("()")]
pub struct CastLeaveGame {
    pub id: u32,
}

impl Handler<CastLeaveGame> for Player {
    type Result = ();

    fn handle(&mut self, message: CastLeaveGame, context: &mut Self::Context) -> Self::Result {
        context.text(vec!["cast:leave_game", &message.id.to_string()].join("\n"));
    }
}
