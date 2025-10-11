use std::time::{Duration, Instant};

use actix::prelude::*;
use actix_web_actors::ws;

use crate::{actors, duck::Duck, messages, protos::protos::protos};
use protobuf::Message;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// A player actor, spawned for each client connection
///
/// Contains a player id, a heartbeat for connection, and game server address
///
/// `Player` accepts stream from client and communicates with `GameServer` actor

#[derive(Debug)]
pub struct Player {
    pub id: u32,
    pub last_heartbeat_time: Instant,
    pub server_address: Addr<actors::game_server::GameServer>,
}

impl Player {
    fn heartbeat(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |actor, context| {
            // check client heartbeats
            if Instant::now().duration_since(actor.last_heartbeat_time) > CLIENT_TIMEOUT {
                log::info!(
                    "Websocket Client ({}) heartbeat failed, disconnecting!",
                    actor.id
                );
                actor
                    .server_address
                    .do_send(messages::LeaveGame { id: actor.id });
                context.stop();
                return;
            }

            context.ping(b"");
        });
    }
}

impl Actor for Player {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, context: &mut Self::Context) {
        self.heartbeat(context);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        // notify game server
        self.server_address
            .do_send(messages::LeaveGame { id: self.id });
        Running::Stop
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Player {
    /// Handles logic when receiving messages from websocket
    fn handle(
        &mut self,
        message: Result<ws::Message, ws::ProtocolError>,
        context: &mut Self::Context,
    ) {
        let message = match message {
            Err(_) => {
                context.stop();
                return;
            }
            Ok(message) => message,
        };

        log::debug!("WEBSOCKET MESSAGE: {message:?}");
        match message {
            ws::Message::Ping(message) => {
                self.last_heartbeat_time = Instant::now();
                context.pong(&message);
            }
            ws::Message::Pong(_) => {
                self.last_heartbeat_time = Instant::now();
            }
            ws::Message::Text(text) => {
                let message = text.trim();

                let v: Vec<&str> = message.splitn(100, '\n').collect();

                match v[0] {
                    "join_game" => {
                        let name = v[1].to_owned();
                        let variety = v[2].to_owned();
                        let color = v[3].to_owned();

                        self.server_address.do_send(messages::JoinGame {
                            player_address: context.address(),
                            name,
                            variety,
                            color,
                        });

                        log::info!("joined: {v:?}");
                    }
                    "vote_start_game" => {
                        // TODO implement vote start system instead
                        self.server_address.do_send(messages::StartGame {});
                    }
                    _ => {}
                }
            }
            ws::Message::Binary(bytes) => {
                let in_message = protos::Duck::parse_from_bytes(&bytes).unwrap();
                self.server_address.do_send(messages::Update {
                    id: self.id,
                    duck: Duck {
                        x: in_message.x,
                        y: in_message.y,
                        z: in_message.z,
                        rotation_radians: in_message.rotation,
                        ..Duck::new()
                    },
                });
            }
            ws::Message::Close(reason) => {
                context.close(reason);
                context.stop();
            }
            ws::Message::Continuation(_) => {
                context.stop();
            }
            ws::Message::Nop => (),
        }
    }
}
