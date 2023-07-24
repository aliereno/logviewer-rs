use std::sync::Arc;
use std::sync::Mutex;

use actix_files as fs;
use actix_web::middleware::Logger;
use actix_web::rt::spawn;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use fetcher::run_background_task;
use model::AppConfig;
use model::BackgroundData;
use model::LogIndexer;
use model::Source;

mod api;
mod fetcher;
mod model;
mod indexer;
use actix_web::rt::time;
use core::time::Duration;

use peak_alloc::PeakAlloc;

#[global_allocator]
static PEAK_ALLOC: PeakAlloc = PeakAlloc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Read the YAML file and parse it
    let yaml_config = std::fs::read_to_string("logviewer_config.yaml").expect("Failed to read logviewer_config.yaml");
    let app_config: AppConfig = serde_yaml::from_str(&yaml_config).expect("Failed to parse YAML");

    let indexer = LogIndexer::new(&app_config.clone().index_dir).expect("error on indexer path init");

    // Create shared data structure
    let shared_data = Arc::new(Mutex::new(BackgroundData {
        log_indexer: indexer,
        sources: Source::from_config(app_config.clone()),
    }));

    // background tasks
    let shared_data_clone = shared_data.clone();
    spawn(async move { run_background_task(shared_data_clone).await });
    spawn(async move {
        let mut interval = time::interval(Duration::from_secs(2));
        loop {
            interval.tick().await;

            let current_mem = PEAK_ALLOC.current_usage_as_mb();
            println!("This program currently uses {} MB of RAM.", current_mem);
        }
    });

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(shared_data.clone()))
            .configure(api::config_app)
            .service(fs::Files::new("/", "./ui").index_file("index.html"))
            .wrap(Logger::default())
    })
    .workers(2)
    .bind((app_config.host, app_config.port))?
    .run()
    .await
}
