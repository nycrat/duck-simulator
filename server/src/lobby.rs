use std::{
    collections::{HashMap, HashSet},
    time::Duration,
};

#[derive(Debug)]
pub struct Lobby {
    pub duck_map: HashMap<u32, (String, String, String)>,
    pub spectator_ids: HashSet<u32>,
    pub bread_list: Vec<(f32, f32, f32)>,
    pub start_time: Option<std::time::SystemTime>,
    pub current_time: std::time::SystemTime,
    pub game_duration: Duration,
}

impl Lobby {
    pub fn new() -> Self {
        Self {
            duck_map: HashMap::new(),
            spectator_ids: HashSet::new(),
            bread_list: Vec::new(),
            start_time: None,
            current_time: std::time::SystemTime::now(),
            game_duration: Duration::from_secs(120),
        }
    }
}
