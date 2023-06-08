use std::error::Error;
use serde_json::json;
use tantivy::IndexSettings;
use tantivy::IndexSortByField;
use tantivy::Order;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::Index;
use tantivy::collector::Count;
use crate::model::{LogIndexer};


impl LogIndexer {
    pub fn new(index_dir: &str) -> Result<Self, Box<dyn Error>> {
        
        // Create the schema for your log index
        let mut schema_builder = SchemaBuilder::new();
        let id_field = schema_builder.add_text_field("id", STRING | STORED);
        let source_id_field = schema_builder.add_i64_field("source_id", INDEXED | FAST);
        let order_field = schema_builder.add_i64_field("order", INDEXED | FAST);
        let log_field = schema_builder.add_text_field("log", TEXT | STORED);
        // Add other fields to the schema as needed
        let schema = schema_builder.build();

        let settings = IndexSettings {
            sort_by_field: Some(IndexSortByField {
                field: "order".to_string(),
                order: Order::Desc,
            }),
            ..Default::default()
        };

        let mut index_builder = Index::builder().schema(schema.clone());
        index_builder = index_builder.settings(settings);

        // Create or open the index
        //let index = Index::create_from_tempdir(schema.clone())?;
        let index = match index_builder.create_in_dir(&index_dir) {
            Ok(i) => i,
            Err(_) => Index::open_in_dir(index_dir)?,
        }; 
        /*
        let index = index_builder.create_in_ram().unwrap();
        */

        // Create a writer for adding documents to the index
        let writer = index.writer(200_000_000)?;

        Ok(LogIndexer {
            index,
            writer,
            schema,
            id_field,
            source_id_field,
            order_field,
            log_field,
            // Initialize other fields as needed
        })
    }

    pub fn add_logs(&mut self, source_id: i32, logs: Vec<String>) -> Result<(), Box<dyn Error>> {
        let mut count = 0;
        for (index, log) in logs.iter().enumerate() {
            let doc = self.log_to_document(source_id, log.to_string(), index);
            self.writer.add_document(doc)?;
            count += 1;
        }
        println!("addlogs : {}", count);
        self.writer.commit()?;
        Ok(())
    }

    pub fn search_logs(&self, source_id: i32, page: usize, page_size: usize) -> Result<(Vec<serde_json::Value>, usize), Box<dyn Error>> {
        let reader = self.index.reader()?;
        let searcher = reader.searcher();
    
        let query_parser = QueryParser::for_index(&self.index, vec![self.source_id_field, self.log_field]);
        let user_query = query_parser.parse_query(&format!("source_id:\"{}\"", source_id))?;
        
        
        let offset = (page - 1) * page_size;
        let limit = page_size;
        
        let mut results: Vec<serde_json::Value> = vec![];
        let total_count = searcher.search(&user_query, &Count).unwrap();
        println!("logs total_count {}", total_count);
        println!("searching logs");

        let top_docs = TopDocs::with_limit(limit).and_offset(offset);
        
        let search_results = searcher.search(&user_query, &(top_docs))?;

        for (_score, doc_address) in search_results {
            let _searcher = searcher.doc(doc_address)?;
            let log_message = _searcher.get_first(self.log_field);
            let id = _searcher.get_first(self.id_field);

            if log_message.is_some() && id.is_some() {
                match (log_message.unwrap().as_text(), id.unwrap().as_text()) {
                    (Some(l), Some(i)) => {                    
                        results.push(json!({
                            "id": i,
                            "message": l,
                        }));
                    },
                    (_, _) => ()
                }
            }
        }

        Ok((results, total_count))
    }

    pub fn log_to_document(&self, source_id: i32, log: String, index: usize) -> Document {
        let mut doc = Document::new();
        doc.add_text(self.id_field, format!("{}#{}", source_id, index));
        doc.add_i64(self.source_id_field, source_id.into());
        doc.add_i64(self.order_field, index.try_into().unwrap());
        doc.add_text(self.log_field, log);
        // Add other fields to the document as needed
        doc
    }
}
