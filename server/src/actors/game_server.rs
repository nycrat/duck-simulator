use crate::{
    duck::Duck,
    lobby,
    messages::{self},
    protos::protos::protos,
};
use protobuf::Message;
use std::{collections::HashMap, f32::consts::PI, time::Duration};

const UPDATE_SYNC_INTERVAL: Duration = Duration::from_millis(50);
const BREAD_SPAWN_PER_SECOND: f32 = 3.0;
const BREAD_LIMIT: usize = 500;

use actix::prelude::*;
use rand::{rngs::ThreadRng, Rng};

#[derive(Debug)]
pub struct GameServer {
    pub clients: HashMap<u32, Recipient<messages::GameMessage>>,
    pub ducks: HashMap<u32, Duck>,
    pub lobbies: HashMap<String, lobby::Lobby>,
    pub rng: ThreadRng,
}

impl GameServer {
    pub fn new() -> GameServer {
        // NOTE default lobby is "main"
        // TODO support more than one lobby and no default lobby maybe
        let mut lobbies = HashMap::new();
        lobbies.insert("main".to_owned(), lobby::Lobby::new());

        GameServer {
            clients: HashMap::new(),
            lobbies,
            rng: rand::thread_rng(),
            ducks: HashMap::new(),
        }
    }

    fn update_sync(&mut self, ctx: &mut Context<Self>) {
        ctx.run_interval(UPDATE_SYNC_INTERVAL, |server_actor, _context| {
            let lobbies: Vec<String> = server_actor.lobbies.keys().map(|x| x.to_owned()).collect();
            let mut resets: Vec<String> = vec![];

            for lobby_name in &lobbies {
                if let Some(lobby) = server_actor.lobbies.get_mut(lobby_name) {
                    if lobby.start_time.is_none() {
                        // TODO REFACTOR THIS BETTER
                        let mut out_msg = protos::UpdateSync::new();
                        out_msg.ducks = server_actor
                            .ducks
                            .iter()
                            .map(|(id, state)| {
                                let mut duck = protos::Duck::new();
                                duck.id = *id;
                                duck.x = state.x;
                                duck.y = state.y;
                                duck.z = state.z;
                                duck.rotation = state.rotation_radians;
                                duck.score = state.score;
                                duck
                            })
                            .collect();
                        server_actor.send_binary_message_to_lobby(
                            lobby_name,
                            out_msg.write_to_bytes().unwrap(),
                            0,
                        );
                        continue;
                    }

                    let delta_time = lobby.current_time.elapsed().unwrap().as_secs_f32();
                    lobby.current_time = std::time::SystemTime::now();

                    let game_over = lobby
                        .current_time
                        .duration_since(lobby.start_time.unwrap())
                        .unwrap()
                        >= lobby.game_duration;

                    if game_over {
                        lobby.start_time = None;
                        resets.push(lobby_name.to_owned());

                        let mut highest_scores = vec![(0, 0); 3.min(lobby.duck_map.len())];

                        for (id, _) in &lobby.duck_map {
                            let duck = server_actor.ducks.get_mut(id).unwrap();
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
                                println!("WHYY");
                                continue;
                            }
                            let duck = server_actor.ducks.get_mut(&id).unwrap();
                            println!("{}: {id}", highest_scores[i].0);
                            duck.x = -1.25 + i as f32 * 1.25;
                            duck.y = 0.0;
                            duck.z = -0.5;
                            duck.rotation_radians = 0.0;
                        }

                        println!("ENDED GAME FOR LOBBY {}", lobby_name);
                    } else {
                        // UPDATE BREAD
                        for (_, y, _) in &mut lobby.bread_list {
                            let gravity = -5.0;
                            // sqrt(v^2 - 2as) = u
                            let velocity = -f32::sqrt(f32::abs(2.0 * gravity * (10.0 - *y)));
                            *y += velocity * delta_time + 0.5 * gravity * delta_time.powi(2);
                            *y = y.max(0.1);
                        }

                        // INTERSECTIONS
                        for (id, _) in &lobby.duck_map {
                            let duck = server_actor.ducks.get_mut(id).unwrap();
                            let duck_pos = &(duck.x, duck.y, duck.z);

                            let duck_size = &(0.5, 0.5, 0.5);
                            let bread_size = &(0.2, 0.2, 0.2);

                            let mut i = 0;
                            while i < lobby.bread_list.len() {
                                let bread_pos = lobby.bread_list.get(i).unwrap();

                                type Vec3 = (f32, f32, f32);
                                fn intersect(
                                    a: &Vec3,
                                    b: &Vec3,
                                    a_size: &Vec3,
                                    b_size: &Vec3,
                                ) -> bool {
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

                            // UPDATE DUCKS
                            // let delta_x = f32::sin(duck.rotation) * 3.0;
                            // let delta_z = f32::cos(duck.rotation) * 3.0;
                            // duck.x += delta_x * delta_time;
                            // duck.z += delta_z * delta_time;
                        }
                    }

                    let mut out_msg = protos::UpdateSync::new();
                    out_msg.ducks = server_actor
                        .ducks
                        .iter()
                        .map(|(id, state)| {
                            let mut duck = protos::Duck::new();
                            duck.id = *id;
                            duck.x = state.x;
                            duck.y = state.y;
                            duck.z = state.z;
                            duck.rotation = state.rotation_radians;
                            duck.score = state.score;
                            duck
                        })
                        .collect();

                    if server_actor.rng.gen_range(0.0..=1.0)
                        <= (BREAD_SPAWN_PER_SECOND * UPDATE_SYNC_INTERVAL.as_secs_f32())
                        && lobby.bread_list.len() < BREAD_LIMIT
                    {
                        let y = 10.0;

                        let theta = server_actor.rng.gen_range(0.0..(PI * 2.0));
                        let r = server_actor.rng.gen_range(0.0..11.5);

                        let x = f32::sin(theta) * r;
                        let z = f32::cos(theta) * r;

                        out_msg.bread_x = Some(x);
                        out_msg.bread_y = Some(y);
                        out_msg.bread_z = Some(z);

                        lobby.bread_list.push((x, y, z));
                    }

                    // println!("BINARY: {:?}", out_msg.write_to_bytes().unwrap().size());
                    server_actor.send_binary_message_to_lobby(
                        lobby_name,
                        out_msg.write_to_bytes().unwrap(),
                        0,
                    );
                    if game_over {
                        server_actor.send_message_to_lobby(&lobby_name, "/game_end", 0);
                    }
                }
            }
            for reset in &resets {
                server_actor.lobbies.remove(reset);
                server_actor
                    .lobbies
                    .insert(reset.to_owned(), lobby::Lobby::new());
            }
        });
    }

    pub fn send_message_to_lobby(&self, lobby: &str, message: &str, skip_id: u32) {
        if let Some(lobby) = self.lobbies.get(lobby) {
            for (id, _) in &lobby.duck_map {
                if *id != skip_id {
                    if let Some(addr) = self.clients.get(&id) {
                        addr.do_send(messages::GameMessage(Some(message.to_owned()), None));
                    }
                }
            }
            for id in &lobby.spectator_ids {
                if *id != skip_id {
                    if let Some(addr) = self.clients.get(&id) {
                        addr.do_send(messages::GameMessage(Some(message.to_owned()), None));
                    }
                }
            }
        }
    }

    /// Broadcasts message to all clients connected to lobby (except skip_id)
    fn send_binary_message_to_lobby(&self, lobby: &str, message: Vec<u8>, skip_id: u32) {
        if let Some(lobby) = self.lobbies.get(lobby) {
            for (id, _) in &lobby.duck_map {
                if *id != skip_id {
                    if let Some(addr) = self.clients.get(&id) {
                        addr.do_send(messages::GameMessage(None, Some(message.clone())));
                    }
                }
            }
            for id in &lobby.spectator_ids {
                if *id != skip_id {
                    if let Some(addr) = self.clients.get(&id) {
                        addr.do_send(messages::GameMessage(None, Some(message.clone())));
                    }
                }
            }
        }
    }
}

impl Actor for GameServer {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.update_sync(ctx);
    }
}
