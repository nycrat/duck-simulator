use actix::prelude::*;
use rand::Rng;

use crate::{
    actors::{GameServer, Player},
    duck::Duck,
    messages::start_game::CastStartGame,
};

/// A message to `GameServer` actor that new player has joined
///
/// Gives address of `Player` actor and name, variety, color of duck
#[derive(Message, Clone)]
#[rtype("()")]
pub struct JoinGame {
    pub player_address: Addr<Player>,
    pub name: String,
    pub variety: String,
    pub color: String,
}

impl GameServer {
    fn handle_join_game(&mut self, message: JoinGame) {
        // TODO use better id generation
        let id = self.rng.gen::<u32>();

        // set player actor as spectator
        if self.start_time.is_some() {}

        self.player_actors.iter().for_each(|(player_id, player)| {
            // notify existing actors of new duck
            player.do_send(CastJoinGame {
                id: id,
                name: message.name.clone(),
                variety: message.variety.clone(),
                color: message.color.clone(),
            });
            let duck = self.ducks.get(player_id).unwrap();

            // notify new duck of existing ducks
            if self.ducks.contains_key(player_id) {
                message.player_address.do_send(CastJoinGame {
                    id: *player_id,
                    name: duck.name.clone().unwrap_or_default(),
                    variety: duck.variety.clone().unwrap_or_default(),
                    color: duck.color.clone().unwrap_or_default(),
                });
            }
        });

        message.player_address.do_send(ReJoinGame { id });

        self.player_actors.insert(id, message.player_address);
        self.ducks.insert(
            id,
            Duck {
                name: Some(message.name),
                variety: Some(message.variety),
                color: Some(message.color),
                ..Duck::new()
            },
        );
    }

    fn handle_spectate_game(&mut self, message: JoinGame) {
        // TODO use better id generation
        let id = self.rng.gen::<u32>();

        // notify spectator of existing ducks
        self.ducks.iter().for_each(|(duck_id, duck)| {
            message.player_address.do_send(CastJoinGame {
                id: *duck_id,
                name: duck.name.clone().unwrap_or_default(),
                variety: duck.variety.clone().unwrap_or_default(),
                color: duck.color.clone().unwrap_or_default(),
            });
        });

        message.player_address.do_send(ReSpectateGame {});
        message.player_address.do_send(CastStartGame {
            start_time: self.start_time.unwrap(),
            game_duration: self.game_duration,
        });

        self.player_actors.insert(id, message.player_address);
        self.spectator_ids.insert(id);
    }
}

impl Handler<JoinGame> for GameServer {
    type Result = ();

    fn handle(&mut self, message: JoinGame, _: &mut Context<Self>) -> Self::Result {
        if self.start_time.is_some() {
            self.handle_spectate_game(message)
        } else {
            self.handle_join_game(message)
        }
    }
}

/// A response message to `Player` actor to communicate the duck's given id
#[derive(Message)]
#[rtype("()")]
pub struct ReJoinGame {
    pub id: u32,
}

impl Handler<ReJoinGame> for Player {
    type Result = ();

    fn handle(&mut self, message: ReJoinGame, context: &mut Self::Context) -> Self::Result {
        self.id = message.id;
        context.text(vec!["re:join_game", &message.id.to_string()].join("\n"));
    }
}

/// A response message to `Player` actor to communicate they are spectating
#[derive(Message)]
#[rtype("()")]
pub struct ReSpectateGame;

impl Handler<ReSpectateGame> for Player {
    type Result = ();

    fn handle(&mut self, _message: ReSpectateGame, context: &mut Self::Context) -> Self::Result {
        context.text("re:spectate_game");
    }
}

/// A message to `Player` actor to broadcast a new duck joining
#[derive(Message)]
#[rtype("()")]
pub struct CastJoinGame {
    pub id: u32,
    pub name: String,
    pub variety: String,
    pub color: String,
}

impl Handler<CastJoinGame> for Player {
    type Result = ();

    fn handle(&mut self, message: CastJoinGame, context: &mut Self::Context) -> Self::Result {
        context.text(
            vec![
                "cast:join_game",
                &message.id.to_string(),
                &message.name,
                &message.variety,
                &message.color,
            ]
            .join("\n"),
        );
    }
}
