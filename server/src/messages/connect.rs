use actix::prelude::*;
use rand::Rng;

use crate::actors;
use crate::actors::game_server::GameServer;
use crate::messages;

/// Woah

#[derive(Message)]
#[rtype(u32)]
pub struct Connect {
    pub player_address: Addr<actors::player::Player>,
    pub name: String,
    pub variety: String,
    pub color: String,
}

impl Handler<Connect> for GameServer {
    type Result = u32;

    fn handle(&mut self, connect_message: Connect, _: &mut Context<Self>) -> Self::Result {
        // TODO use better id generation
        let id = self.rng.gen::<u32>();

        if self.lobbies.get("main").unwrap().start_time.is_some() {
            connect_message
                .player_address
                .do_send(messages::GameMessage(
                    Some(format!(
                        "/spectate_game\n{}\n{}",
                        self.lobbies.get("main").unwrap().game_duration.as_secs(),
                        self.lobbies
                            .get("main")
                            .unwrap()
                            .start_time
                            .unwrap()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_secs()
                    )),
                    None,
                ));

            connect_message
                .player_address
                .do_send(messages::GameMessage(
                    Some(format!("/id\n{id}").to_owned()),
                    None,
                ));
            {
                let other_infos: String = self
                    .lobbies
                    .get("main")
                    .unwrap()
                    .duck_map
                    .iter()
                    .map(|(id, info)| format!("\n{} {} {} {}", id, info.0, info.1, info.2))
                    .collect();

                connect_message
                    .player_address
                    .do_send(messages::GameMessage(
                        Some(format!("/join{other_infos}").to_owned()),
                        None,
                    ));
            }

            self.player_actors
                .insert(id, connect_message.player_address);
            self.lobbies
                .get_mut("main")
                .unwrap()
                .spectator_ids
                .insert(id);
            return id;
        }

        // notify of existing ducks in lobby
        connect_message
            .player_address
            .do_send(messages::GameMessage(
                Some(format!("/id\n{id}").to_owned()),
                None,
            ));
        {
            let other_infos: String = self
                .lobbies
                .get("main")
                .unwrap()
                .duck_map
                .iter()
                .map(|(id, info)| format!("\n{} {} {} {}", id, info.0, info.1, info.2))
                .collect();

            connect_message
                .player_address
                .do_send(messages::GameMessage(
                    Some(format!("/join{other_infos}").to_owned()),
                    None,
                ));
        }

        self.player_actors
            .insert(id, connect_message.player_address);
        self.ducks.insert(
            id,
            crate::duck::Duck {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                rotation_radians: 0.0,
                score: 0,
            },
        );

        // notify all users in same lobby
        self.send_message_to_lobby(
            "main",
            &format!(
                "/join\n{id} {} {} {}",
                connect_message.name, connect_message.variety, connect_message.color
            ),
            id,
        );

        self.lobbies.get_mut("main").unwrap().duck_map.insert(
            id,
            (
                connect_message.name,
                connect_message.variety,
                connect_message.color,
            ),
        );

        // send id back
        id
    }
}
