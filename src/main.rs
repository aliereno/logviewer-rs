use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;

use actix_files as fs;
use actix_web::middleware::Logger;
use actix_web::rt::spawn;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use model::RwLockStat;
use model::Stat;
use model::TaskManager;
use tasks::print_memory_usage;
use model::AppConfig;
use model::BackgroundData;
use model::LogIndexer;
use model::Source;

mod api;
mod error;
mod fetcher;
mod model;
mod indexer;
mod helper;
mod tasks;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Read the YAML file and parse it
    let yaml_config = std::fs::read_to_string("logviewer_config.yaml").expect("Failed to read logviewer_config.yaml");
    let app_config: AppConfig = serde_yaml::from_str(&yaml_config).expect("Failed to parse YAML");

    let rw_stats: RwLockStat = Arc::new(RwLock::new(Stat { ram_usage: 0 as f64, queue_count: 0 as i64 }));
    let indexer = LogIndexer::new(&app_config.clone().index_dir).expect("error on indexer path init");
    let task_manager: TaskManager = TaskManager::new(Arc::new(Mutex::new(indexer.create_writer().unwrap())), rw_stats.clone());
    
    let source_list = Source::from_config(app_config.clone());
    task_manager.send_source_indexing_task_multiple(source_list.clone());

    // Create shared data structure
    let shared_data = Arc::new(Mutex::new(BackgroundData {
        log_indexer: indexer,
        sources: source_list,
        task_manager: Arc::new(task_manager),
        stats: rw_stats.clone()
    }));

    // background tasks
    spawn(async move { print_memory_usage(rw_stats).await });

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(shared_data.clone()))
            .configure(api::config_app)
            .service(fs::Files::new("/", "./ui").index_file("index.html"))
            .wrap(Logger::default())
    })
    .bind((app_config.host, app_config.port))?
    .run()
    .await
}
