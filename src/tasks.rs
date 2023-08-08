use actix_web::rt::time;
use core::time::Duration;
use std::error::Error;

use crate::{model::{ArcMutexBackgroundData, LogIndexer, Source}, fetcher::fetch_data_from_file};

use peak_alloc::PeakAlloc;

#[global_allocator]
static PEAK_ALLOC: PeakAlloc = PeakAlloc;


pub fn run_indexer_by_source_id(indexer: &mut LogIndexer, source: Source) -> Result<(), Box<dyn Error>> {

    indexer.delete_indexes_by_source_id(source.id).unwrap();

    fetch_data_from_file(source.clone(), indexer);

    Ok(())
}

pub async fn run_indexer_in_background(shared_data: ArcMutexBackgroundData) {
    let mut interval = time::interval(Duration::from_secs(1800));

    loop {
        interval.tick().await;
        println!("started");
        let mut data = shared_data.lock().unwrap();

        for source in &mut data.sources.clone() {
            let _ = run_indexer_by_source_id(&mut data.log_indexer, source.clone());
        }

        println!("finished");
    }
}

pub async fn print_memory_usage() {
    let mut interval = time::interval(Duration::from_secs(2));
    loop {
        interval.tick().await;

        let current_mem = PEAK_ALLOC.current_usage_as_mb();
        println!("This program currently uses {} MB of RAM.", current_mem);
    }
}