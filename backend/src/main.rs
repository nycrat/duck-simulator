#![warn(missing_docs)]
#![doc = include_str!("../readme.md")]

use actix::*;
use actix_web::{middleware::Logger, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use std::time::Instant;

mod actors;
mod duck;
mod messages;
mod protos;

/// Spawns a player actor linked to the websocket connection
async fn spawn_player_actor(
    request: HttpRequest,
    stream: web::Payload,
    server: web::Data<Addr<actors::GameServer>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        actors::Player {
            id: 0,
            last_heartbeat_time: Instant::now(),
            server_address: server.get_ref().clone(),
        },
        &request,
        stream,
    )
}

/// Starts web server with websocket route /ws for client connection
///
/// Attaches a single game server actor as server state
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let local_ip = local_ip_address::local_ip();
    let host = match local_ip.is_ok() {
        true => local_ip.unwrap().to_string(),
        false => String::from("localhost"),
    };
    let port: i32 = 4421;

    let game_actor_address = actors::GameServer::new().start();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(game_actor_address.clone()))
            .route("/ws", web::get().to(spawn_player_actor))
            .wrap(Logger::default())
    })
    .workers(2)
    .bind(format!("{host}:{port}"))?
    .run()
    .await
}
