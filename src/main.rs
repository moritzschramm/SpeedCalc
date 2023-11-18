use actix_files::Files;
use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

mod socket;
use crate::socket::CalcTaskWebSocket;

const HOSTNAME: &str = "127.0.0.1";     // 0.0.0.0 to expose on network and test on mobile
const PORT: u16 = 8000;

/// starts the web socket
async fn calc_task_ws(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(CalcTaskWebSocket::new(), &req, stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("Starting HTTP server at http://{HOSTNAME}:{PORT}");

    HttpServer::new(|| {
        App::new()
            .service(web::resource("/calc-task-ws").route(web::get().to(calc_task_ws)))     // websocket endpoint
            .service(Files::new("/", "./client/dist/").index_file("index.html"))            // serves html, js and css
            .wrap(middleware::Compress::default())                                          // compress files with gzip
            .wrap(middleware::Logger::default())                                            // enable logging
            
    })
    .bind((HOSTNAME, PORT))?
    .run()
    .await
}
