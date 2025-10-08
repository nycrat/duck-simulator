use crate::{
    actors,
    duck::Duck,
    lobby::Lobby,
    messages::{self},
    protos::protos::protos,
};
use protobuf::{Message, SpecialFields};
use std::{collections::HashMap, f32::consts::PI, time::Duration};

const UPDATE_SYNC_INTERVAL: Duration = Duration::from_millis(50);
const BREAD_SPAWN_PER_SECOND: f32 = 3.0;
const BREAD_LIMIT: usize = 500;

use actix::prelude::*;
use rand::{rngs::ThreadRng, Rng};

/// A game server actor
///
/// Contains state of all ducks and lobbies, and addresses of player actors
///
/// Handles updating world state and communicates with `Player` actor

#[derive(Debug)]
pub struct GameServer {
    pub player_actors: HashMap<u32, Addr<actors::player::Player>>,
    pub ducks: HashMap<u32, Duck>,
    pub lobbies: HashMap<String, Lobby>,
    pub rng: ThreadRng,
}

impl GameServer {
    pub fn new() -> GameServer {
        // NOTE default lobby is "main"
        // TODO support more than one lobby and no default lobby maybe
        let mut lobbies = HashMap::new();
        lobbies.insert("main".to_owned(), Lobby::new());

        GameServer {
            player_actors: HashMap::new(),
            lobbies,
            rng: rand::thread_rng(),
            ducks: HashMap::new(),
        }
    }

    /// Produces UpdateSync proto for the given lobby
    fn get_update_sync_proto(&mut self, lobby_name: &str) -> protos::UpdateSync {
        // TODO REFACTOR THIS BETTER
        let mut message = protos::UpdateSync::new();
        let lobby = self.lobbies.get_mut(lobby_name).unwrap();
        message.ducks = self
            .ducks
            .iter()
            .filter(|(id, _)| lobby.duck_map.contains_key(id))
            .map(|(id, duck)| protos::Duck {
                id: *id,
                rotation: duck.rotation_radians,
                x: duck.x,
                y: duck.y,
                z: duck.z,
                score: duck.score,
                special_fields: SpecialFields::new(),
            })
            .collect();

        message
    }

    /// Updates state of given lobby by one tick
    fn tick_lobby(&mut self, lobby_name: &str, delta_time: f32) {
        // UPDATE BREAD
        let lobby = self.lobbies.get_mut(lobby_name).unwrap();
        for (_, y, _) in &mut lobby.bread_list {
            let gravity = -5.0;
            // sqrt(v^2 - 2as) = u
            let velocity = -f32::sqrt(f32::abs(2.0 * gravity * (10.0 - *y)));
            *y += velocity * delta_time + 0.5 * gravity * delta_time.powi(2);
            *y = y.max(0.1);
        }

        // INTERSECTIONS
        for (id, _) in &lobby.duck_map {
            let duck = self.ducks.get_mut(id).unwrap();
            let duck_pos = &(duck.x, duck.y, duck.z);

            let duck_size = &(0.5, 0.5, 0.5);
            let bread_size = &(0.2, 0.2, 0.2);

            let mut i = 0;
            while i < lobby.bread_list.len() {
                let bread_pos = lobby.bread_list.get(i).unwrap();

                type Vec3 = (f32, f32, f32);
                fn intersect(a: &Vec3, b: &Vec3, a_size: &Vec3, b_size: &Vec3) -> bool {
                    return a.0 - a_size.0 <= b.0 + b_size.0
                        && a.0 + a_size.0 >= b.0 - b_size.0
                        && a.1 - a_size.1 <= b.1 + b_size.1
                        && a.1 + a_size.1 >= b.1 - b_size.1
                        && a.2 - a_size.2 <= b.2 + b_size.2
                        && a.2 + a_size.2 >= b.2 - b_size.2;
                }

                if intersect(duck_pos, bread_pos, duck_size, bread_size) {
                    lobby.bread_list.swap_remove(i);
                    duck.score += 1;
                } else {
                    i += 1;
                }
            }
        }
    }

    /// Appends new bread to lobby if it's started and past bread spawn interval
    ///
    /// Returns the new bread coordinates if spawned, otherwise None
    fn spawn_new_bread(&mut self, lobby: &str) -> Option<(f32, f32, f32)> {
        let lobby = self.lobbies.get_mut(lobby).unwrap();
        if lobby.start_time.is_none() {
            return None;
        }
        if self.rng.gen_range(0.0..=1.0)
            <= (BREAD_SPAWN_PER_SECOND * UPDATE_SYNC_INTERVAL.as_secs_f32())
            && lobby.bread_list.len() < BREAD_LIMIT
        {
            let y = 10.0;

            let theta = self.rng.gen_range(0.0..(PI * 2.0));
            let r = self.rng.gen_range(0.0..11.5);

            let x = f32::sin(theta) * r;
            let z = f32::cos(theta) * r;

            lobby.bread_list.push((x, y, z));

            return Some((x, y, z));
        }
        None
    }

    /// Sets lobby state to podium view
    fn end_game(&mut self, lobby_name: &str) {
        let lobby = self.lobbies.get_mut(lobby_name).unwrap();

        let mut highest_scores = vec![(0, 0); 3.min(lobby.duck_map.len())];

        for (id, _) in &lobby.duck_map {
            let duck = self.ducks.get_mut(id).unwrap();
            for i in 0..highest_scores.len() {
                if duck.score >= highest_scores[i].0 {
                    highest_scores.insert(i, (duck.score, *id));
                    highest_scores.pop();
                    break;
                }
            }
            duck.x = 0.0;
            duck.y = 0.0;
            duck.z = 4.0;
            duck.rotation_radians = 0.0;
        }

        for i in 0..highest_scores.len() {
            let id = highest_scores[i].1;
            if id == 0 {
                log::warn!("ID = 0 ON PODIUM");
                continue;
            }
            let duck = self.ducks.get_mut(&id).unwrap();
            duck.x = -1.25 + i as f32 * 1.25;
            duck.y = 0.0;
            duck.z = -0.5;
            duck.rotation_radians = 0.0;
        }

        log::info!("ENDED GAME FOR LOBBY {}", lobby_name);
    }

    /// Apply updates to all lobbies
    fn update(&mut self) {
        let lobby_name_list: Vec<String> = self.lobbies.keys().cloned().collect();

        for lobby_name in &lobby_name_list {
            let lobby = self.lobbies.get_mut(lobby_name).unwrap();
            let game_over = lobby.start_time.is_some()
                && std::time::SystemTime::now()
                    .duration_since(lobby.start_time.unwrap())
                    .unwrap()
                    >= lobby.game_duration;
            if game_over {
                self.end_game(lobby_name);
            } else {
                let delta_time = lobby.current_time.elapsed().unwrap().as_secs_f32();
                lobby.current_time = std::time::SystemTime::now();
                self.tick_lobby(lobby_name, delta_time);
            }

            let mut update_message = self.get_update_sync_proto(lobby_name);

            if let Some((x, y, z)) = self.spawn_new_bread(lobby_name) {
                update_message.bread_x = Some(x);
                update_message.bread_y = Some(y);
                update_message.bread_z = Some(z);
            }
            self.send_binary_message_to_lobby(
                lobby_name,
                update_message.write_to_bytes().unwrap(),
                0,
            );

            if game_over {
                self.send_message_to_lobby(&lobby_name, "/game_end", 0);
                self.lobbies.remove(lobby_name);
                self.lobbies.insert(lobby_name.to_owned(), Lobby::new());
            }
        }
    }

    /// Broadcasts message to all clients connected to lobby (except skip_id)
    pub fn send_message_to_lobby(&self, lobby: &str, message: &str, skip_id: u32) {
        if let Some(lobby) = self.lobbies.get(lobby) {
            for (id, _) in &lobby.duck_map {
                if *id != skip_id {
                    if let Some(addr) = self.player_actors.get(&id) {
                        addr.do_send(messages::GameMessage(Some(message.to_owned()), None));
                    }
                }
            }
            for id in &lobby.spectator_ids {
                if *id != skip_id {
                    if let Some(addr) = self.player_actors.get(&id) {
                        addr.do_send(messages::GameMessage(Some(message.to_owned()), None));
                    }
                }
            }
        }
    }

    /// Broadcasts binary message to all clients connected to lobby (except skip_id)
    fn send_binary_message_to_lobby(&self, lobby: &str, message: Vec<u8>, skip_id: u32) {
        if let Some(lobby) = self.lobbies.get(lobby) {
            for (id, _) in &lobby.duck_map {
                if *id != skip_id {
                    if let Some(addr) = self.player_actors.get(&id) {
                        addr.do_send(messages::GameMessage(None, Some(message.clone())));
                    }
                }
            }
            for id in &lobby.spectator_ids {
                if *id != skip_id {
                    if let Some(addr) = self.player_actors.get(&id) {
                        addr.do_send(messages::GameMessage(None, Some(message.clone())));
                    }
                }
            }
        }
    }
}

impl Actor for GameServer {
    type Context = Context<Self>;

    fn started(&mut self, context: &mut Self::Context) {
        context.run_interval(UPDATE_SYNC_INTERVAL, |server_actor, _context| {
            server_actor.update();
        });
    }
}
