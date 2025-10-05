use actix::prelude::*;

use crate::actors::game_server::GameServer;

#[derive(Message)]
#[rtype(result = "()")]
pub struct StartLobby {
    pub lobby: String,
    pub game_duration: u64,
}

impl Handler<StartLobby> for GameServer {
    type Result = MessageResult<StartLobby>;

    fn handle(&mut self, msg: StartLobby, _: &mut Self::Context) -> Self::Result {
        self.lobbies.get_mut(&msg.lobby).unwrap().game_duration =
            std::time::Duration::from_secs(msg.game_duration);
        if let Some(lobby) = self.lobbies.get(&msg.lobby) {
            if lobby.start_time.is_none() {
                println!(
                    "STARTED GAME FOR LOBBY {} WITH {} DUCKS WITH DURATION {}",
                    &msg.lobby,
                    self.ducks.len(),
                    lobby.game_duration.as_secs()
                );
                let now = Some(std::time::SystemTime::now());
                self.send_message_to_lobby(
                    &msg.lobby,
                    &format!(
                        "/start_game\n{}\n{}",
                        std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_secs(),
                        lobby.game_duration.as_secs(),
                    ),
                    0,
                );
                self.lobbies.get_mut(&msg.lobby).unwrap().start_time = now;
                // lobby.start_time = now;
            }
        }
        MessageResult(())
    }
}
