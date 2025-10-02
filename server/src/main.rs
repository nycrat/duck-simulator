use std::time::Instant;

use actix::*;
use actix_web::{middleware::Logger, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

mod client;
mod duck;
mod lobby;
mod protos;
mod server;

async fn websocket_route(
    request: HttpRequest,
    stream: web::Payload,
    server: web::Data<Addr<server::GameServer>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        client::Client {
            id: 0,
            last_heartbeat_time: Instant::now(),
            server_address: server.get_ref().clone(),
        },
        &request,
        stream,
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    dotenvy::dotenv().unwrap();

    let local_ip = local_ip_address::local_ip();

    let server = server::GameServer::new().start();

    let host = match local_ip.is_ok() {
        true => local_ip.unwrap().to_string(),
        false => std::env::var("HOST").unwrap(),
    };

    let port: i32 = 4421;

    // TODO update to wss when it supports https
    log::info!("starting game server at ws://{}:{}/ws", host, port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(server.clone()))
            .route("/ws", web::get().to(websocket_route))
            .wrap(Logger::default())
    })
    .workers(2)
    .bind(format!("{host}:{port}"))?
    .run()
    .await
}
