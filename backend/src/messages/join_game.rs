use actix::prelude::*;
use rand::Rng;

use crate::{
    actors::{game_server::GameServer, player::Player},
    duck::Duck,
};

/// Woah

#[derive(Message, Clone)]
#[rtype("()")]
pub struct JoinGame {
    pub player_address: Addr<Player>,
    pub name: String,
    pub variety: String,
    pub color: String,
}

impl Handler<JoinGame> for GameServer {
    type Result = ();

    fn handle(&mut self, message: JoinGame, _: &mut Context<Self>) -> Self::Result {
        // TODO use better id generation
        let id = self.rng.gen::<u32>();

        if self.start_time.is_some() {
            // TODO !!!

            // self.player_actors.insert(id, message.player_address);
            // self.spectator_ids.insert(id);
            // return id;
        }

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
            message.player_address.do_send(CastJoinGame {
                id: *player_id,
                name: duck.name.clone().unwrap_or_default(),
                variety: duck.variety.clone().unwrap_or_default(),
                color: duck.color.clone().unwrap_or_default(),
            });
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
}

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
