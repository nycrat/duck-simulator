use crate::{actors, duck::Duck, messages, protos::protos::protos};
use protobuf::{Message, SpecialFields};
use std::{
    collections::{HashMap, HashSet},
    f32::consts::PI,
    time::{Duration, SystemTime},
};

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
    pub _spectator_ids: HashSet<u32>,
    pub bread_list: Vec<(f32, f32, f32)>,
    pub start_time: Option<std::time::SystemTime>,
    pub current_time: std::time::SystemTime,
    pub game_duration: Duration,
    pub rng: ThreadRng,
}

impl GameServer {
    pub fn new() -> GameServer {
        GameServer {
            player_actors: HashMap::new(),
            rng: rand::thread_rng(),
            ducks: HashMap::new(),
            _spectator_ids: HashSet::new(),
            bread_list: Vec::new(),
            start_time: None,
            current_time: SystemTime::now(),
            game_duration: Duration::from_secs(30),
        }
    }

    /// Produces UpdateSync proto for the given lobby
    fn get_update_sync_proto(&mut self) -> protos::UpdateSync {
        let mut message = protos::UpdateSync::new();
        message.ducks = self
            .ducks
            .iter()
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
    fn tick_game(&mut self, delta_time: f32) {
        // UPDATE BREAD
        for (_, y, _) in &mut self.bread_list {
            let gravity = -5.0;
            // sqrt(v^2 - 2as) = u
            let velocity = -f32::sqrt(f32::abs(2.0 * gravity * (10.0 - *y)));
            *y += velocity * delta_time + 0.5 * gravity * delta_time.powi(2);
            *y = y.max(0.1);
        }

        let duck_ids: Vec<u32> = self.ducks.iter().map(|(id, _)| *id).collect();

        // INTERSECTIONS
        for id in duck_ids {
            let duck = self.ducks.get_mut(&id).unwrap();
            let duck_pos = &(duck.x, duck.y, duck.z);

            let duck_size = &(0.5, 0.5, 0.5);
            let bread_size = &(0.2, 0.2, 0.2);

            let mut i = 0;
            while i < self.bread_list.len() {
                let bread_pos = self.bread_list.get(i).unwrap();

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
                    self.bread_list.swap_remove(i);
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
    fn spawn_new_bread(&mut self) -> Option<(f32, f32, f32)> {
        if self.start_time.is_none() {
            return None;
        }
        if self.rng.gen_range(0.0..=1.0)
            <= (BREAD_SPAWN_PER_SECOND * UPDATE_SYNC_INTERVAL.as_secs_f32())
            && self.bread_list.len() < BREAD_LIMIT
        {
            let y = 10.0;

            let theta = self.rng.gen_range(0.0..(PI * 2.0));
            let r = self.rng.gen_range(0.0..11.5);

            let x = f32::sin(theta) * r;
            let z = f32::cos(theta) * r;

            self.bread_list.push((x, y, z));

            return Some((x, y, z));
        }
        None
    }

    /// Sets lobby state to podium view
    fn end_game(&mut self) {
        let mut highest_scores = vec![(0, 0); usize::min(self.ducks.len(), 3)];

        let duck_ids: Vec<u32> = self.ducks.iter().map(|(id, _)| *id).collect();

        for id in duck_ids {
            let duck = self.ducks.get_mut(&id).unwrap();
            for i in 0..highest_scores.len() {
                if duck.score >= highest_scores[i].0 {
                    highest_scores.insert(i, (duck.score, id));
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

        log::info!("ENDED GAME");
    }

    /// Apply updates to all lobbies
    fn update(&mut self) {
        let game_over = self.start_time.is_some()
            && std::time::SystemTime::now()
                .duration_since(self.start_time.unwrap())
                .unwrap()
                >= self.game_duration;
        if game_over {
            self.end_game();
        } else {
            let delta_time = self.current_time.elapsed().unwrap().as_secs_f32();
            self.current_time = std::time::SystemTime::now();
            self.tick_game(delta_time);
        }

        let mut update_message = self.get_update_sync_proto();

        if let Some((x, y, z)) = self.spawn_new_bread() {
            update_message.bread_x = Some(x);
            update_message.bread_y = Some(y);
            update_message.bread_z = Some(z);
        }
        let update_data = update_message.write_to_bytes().unwrap();

        // PERF having to clone this is something to look at improving
        self.player_actors.iter().for_each(|(_, player)| {
            player.do_send(messages::CastUpdateGame {
                update_data: update_data.clone(),
            });
        });

        if game_over {
            self.player_actors.iter().for_each(|(_, player)| {
                player.do_send(messages::CastEndGame {});
            });
            self.start_time = None;
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
