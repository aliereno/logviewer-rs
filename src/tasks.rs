use actix_web::rt::time;
use core::time::Duration;
use std::{sync::{mpsc::{Sender, Receiver, channel}, Arc, Mutex}, thread};

use crate::{model::{Task, SourceIndexingTask, TaskManager, Source, MutexIndexWriter}, fetcher::fetch_data_from_file};

use peak_alloc::PeakAlloc;

#[global_allocator]
static PEAK_ALLOC: PeakAlloc = PeakAlloc;


pub async fn print_memory_usage() {
    let mut interval = time::interval(Duration::from_secs(2));
    loop {
        interval.tick().await;

        let current_mem = PEAK_ALLOC.current_usage_as_mb();
        println!("This program currently uses {} MB of RAM.", current_mem);
    }
}

impl Task for SourceIndexingTask {
    fn execute(&self, index_writer_mutex: &mut MutexIndexWriter) {
        let mut index_writer = index_writer_mutex.lock().unwrap();
        index_writer.delete_indexes_by_source_id(self.source.id).unwrap();

        let fields = index_writer.fields.clone();

        fetch_data_from_file(self.source.clone(), &mut index_writer.writer, fields);
    }
}

impl TaskManager {
    pub fn new(index_writer: MutexIndexWriter) -> Self {
        let (sender, receiver): (Sender<Box<dyn Task>>, Receiver<Box<dyn Task>>) = channel();
        let sender = Arc::new(Mutex::new(sender));
        
        let index_writer_clone = index_writer.clone();
        // Start a thread to process tasks from the channel
        thread::spawn(move || {
            for task in receiver {
                let mut index_writer_clone = index_writer.clone(); 
                task.execute(&mut index_writer_clone);
            }
        });

        TaskManager { sender, index_writer: index_writer_clone }
    }

    pub fn send_source_indexing_task(&self, source: Source) {
        // Send the task to the processing thread
        self.sender.lock().unwrap().send(Box::new(SourceIndexingTask{source})).expect("Failed to send task");
    }

    pub fn send_source_indexing_task_multiple(&self, source_list: Vec<Source>) {
        // Send the task to the processing thread
        let sender = self.sender.lock().unwrap();
        
        for source in source_list {
            let result = sender.send(Box::new(SourceIndexingTask{source}));

            if let Err(e) = result {
                eprintln!("Failed to send source task: {}", e);
            }
        }
    }
}
