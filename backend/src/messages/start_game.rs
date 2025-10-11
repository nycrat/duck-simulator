use actix::prelude::*;

use crate::actors::{game_server::GameServer, player::Player};

/// A message to start game sent to `GameServer` actor
///
/// `GameServer` actor sends `GameMessage` to start game for each
/// `Player` actor in lobby

#[derive(Message)]
#[rtype(result = "()")]
pub struct StartGame {}

impl Handler<StartGame> for GameServer {
    type Result = ();

    fn handle(&mut self, _: StartGame, _: &mut Self::Context) -> Self::Result {
        if self.start_time.is_none() {
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

#[derive(Message)]
#[rtype(result = "()")]
pub struct CastStartGame {
    start_time: std::time::SystemTime,
    game_duration: std::time::Duration,
}

impl Handler<CastStartGame> for Player {
    type Result = ();

    fn handle(&mut self, message: CastStartGame, context: &mut Self::Context) -> Self::Result {
        let start_time = message
            .start_time
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let game_duration = message.game_duration.as_secs();
        context.text(
            vec![
                "cast:start_game",
                &start_time.to_string(),
                &game_duration.to_string(),
            ]
            .join("\n"),
        );
    }
}
