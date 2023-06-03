use std::sync::Arc;
use std::sync::Mutex;

use actix_files as fs;
use actix_web::middleware::Logger;
use actix_web::rt::spawn;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use fetcher::run_background_task;
use model::BackgroundData;
use model::Source;

mod api;
mod fetcher;
mod model;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let log_paths = std::env::var("LOG_PATHS").expect("env `LOG_PATHS` must be set.");
    

    // Create shared data structure
    let shared_data = Arc::new(Mutex::new(BackgroundData {
        // Initialize the data fields as needed
        sources: Source::from_env(log_paths),
    }));

    // background tasks
    let shared_data_clone = shared_data.clone();
    spawn(async move {
        run_background_task(shared_data_clone).await
    });

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(shared_data.clone()))
            .configure(api::config_app)
            .service(fs::Files::new("/", "./static").index_file("index.html"))
            .wrap(Logger::default())
    })
    .workers(1)
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
