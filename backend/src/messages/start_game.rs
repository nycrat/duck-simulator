use actix::prelude::*;

use crate::actors::Player;

/// A message to `Player` actor to broadcast game starting
#[derive(Message)]
#[rtype("()")]
pub struct CastStartGame {
    pub start_time: std::time::SystemTime,
    pub game_duration: std::time::Duration,
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
