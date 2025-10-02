use std::time::{Duration, Instant};

use actix::prelude::*;
use actix_web_actors::ws;

use crate::{duck::Duck, protos::protos::protos, server};
use protobuf::Message;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Debug)]
pub struct Client {
    pub id: u32,
    pub last_heartbeat_time: Instant,
    pub server_address: Addr<server::GameServer>,
}

impl Client {
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
                    .do_send(server::Disconnect { id: actor.id });
                context.stop();
                return;
            }

            context.ping(b"");
        });
    }
}

impl Actor for Client {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, context: &mut Self::Context) {
        self.heartbeat(context);

        // let addr = ctx.address();
        // self.addr
        //     .send(server::Connect {
        //         addr: addr.recipient(),
        //     })
        //     .into_actor(self)
        //     .then(|res, act, ctx| {
        //         match res {
        //             Ok(res) => act.id = res,
        //             // something is wrong with chat server
        //             _ => ctx.stop(),
        //         }
        //         fut::ready(())
        //     })
        //     .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        // notify game server
        self.server_address
            .do_send(server::Disconnect { id: self.id });
        Running::Stop
    }
}

impl Handler<server::GameMessage> for Client {
    type Result = ();

    fn handle(&mut self, message: server::GameMessage, context: &mut Self::Context) {
        if message.0.is_some() {
            context.text(message.0.unwrap());
        }
        if message.1.is_some() {
            context.binary(message.1.unwrap());
        }
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Client {
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

                if message.starts_with('/') {
                    let v: Vec<&str> = message.splitn(2, ' ').collect();
                    match v[0] {
                        "/list" => self
                            .server_address
                            .send(server::ListLobbies)
                            .into_actor(self)
                            .then(|res, _, ctx| {
                                match res {
                                    Ok(lobbies) => {
                                        let lobbies: String = lobbies
                                            .into_iter()
                                            .map(|lobby| "\n".to_owned() + &lobby)
                                            .collect();
                                        ctx.text(format!("/list{lobbies}"));
                                    }
                                    _ => println!("Something is wrong"),
                                }
                                fut::ready(())
                            })
                            .wait(context),
                        "/join" => {
                            if v.len() == 2 {
                                // name duck color
                                log::info!("JOINED");
                                self.server_address.do_send(server::Join {
                                    id: self.id,
                                    name: v[1].to_owned(),
                                });
                            }
                        }
                        "/info" => {
                            let duck_info: Vec<&str> = v[1].splitn(3, " ").collect();
                            let name = duck_info[0].to_owned();
                            let variety = duck_info[1].to_owned();
                            let color = duck_info[2].to_owned();

                            self.server_address
                                .send(server::Connect {
                                    recipient: context.address().recipient(),
                                    name,
                                    variety,
                                    color,
                                })
                                .into_actor(self)
                                .then(|res, act, ctx| {
                                    match res {
                                        Ok(res) => act.id = res,
                                        _ => ctx.stop(),
                                    }
                                    fut::ready(())
                                })
                                .wait(context);

                            log::info!("joined: {duck_info:?}");
                        }
                        "/start_game" => {
                            let (lobby, game_duration) = v[1].split_once(" ").unwrap();
                            self.server_address.do_send(server::StartLobby {
                                lobby: lobby.to_owned(),
                                game_duration: game_duration.parse().unwrap(),
                            });
                        }
                        _ => context.text(format!("/{message:?}")),
                    }
                } else {
                }
            }
            ws::Message::Binary(bytes) => {
                let in_message = protos::Duck::parse_from_bytes(&bytes).unwrap();
                self.server_address.do_send(server::Update {
                    id: self.id,
                    duck: Duck {
                        x: in_message.x,
                        y: in_message.y,
                        z: in_message.z,
                        rotation_radians: in_message.rotation,
                        score: 0,
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
