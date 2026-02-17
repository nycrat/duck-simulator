use actix::prelude::*;

use crate::actors::GameServer;
use crate::messages::CastStartGame;

/// Message to vote for starting the game
#[derive(Message)]
#[rtype("()")]
pub struct VoteStartGame {
    pub id: u32,
}

impl Handler<VoteStartGame> for GameServer {
    type Result = ();

    fn handle(&mut self, message: VoteStartGame, _context: &mut Self::Context) -> Self::Result {
        if self.start_time.is_some() {
            return;
        }

        self.votes.insert(message.id);

        let total_players = self.player_actors.len();
        let votes_needed = (total_players + 1) / 2;

        log::info!(
            "Vote received from player {}. Votes: {}/{} (need {})",
            message.id,
            self.votes.len(),
            total_players,
            votes_needed
        );

        if self.votes.len() >= votes_needed {
            log::info!(
                "STARTED GAME WITH {} DUCKS WITH DURATION {}",
                self.ducks.len(),
                self.game_duration.as_secs()
            );
            let start_time = std::time::SystemTime::now();
            let game_duration = self.game_duration;

            self.player_actors.iter().for_each(|(_, player)| {
                player.do_send(CastStartGame {
                    start_time,
                    game_duration,
                })
            });

            self.start_time = Some(start_time);
        }
    }
}
