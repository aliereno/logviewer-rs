use core::time::Duration;

use actix_web::middleware::Logger;
use actix_web::rt::spawn;
use actix_web::{App, HttpServer};
use actix_web::rt::time;
use actix_files as fs;

mod fetcher;
mod api;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // background tasks
    spawn(async move {
        let mut interval = time::interval(Duration::from_secs(10));
        loop {
            interval.tick().await;
            // do something
        }
    });

    HttpServer::new(move || {
        App::new()
            .configure(api::config_app)
            .service(fs::Files::new("/", "./static").index_file("index.html"))
            .wrap(Logger::default())
    })
    .workers(1)
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}