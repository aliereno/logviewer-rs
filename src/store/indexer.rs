use std::error::Error;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::Index;
use tantivy::collector::Count;
use crate::model::{LogIndexer, LogJson};


impl LogIndexer {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        
        // Create the schema for your log index
        let mut schema_builder = SchemaBuilder::new();
        let source_id_field = schema_builder.add_i64_field("source_id", INDEXED);
        let log_field = schema_builder.add_json_field("log", TEXT | STORED);
        // Add other fields to the schema as needed
        let schema = schema_builder.build();

        /*
        // Create or open the index
        //let index = Index::create_from_tempdir(schema.clone())?;
        let index = match Index::create_in_dir(&index_dir, schema.clone()) {
            Ok(i) => i,
            Err(_) => Index::open_in_dir(index_dir)?,
        }; 
        */
        let index = Index::create_in_ram(schema.clone());

        // Create a writer for adding documents to the index
        let writer = index.writer(200_000_000)?;

        Ok(LogIndexer {
            index,
            writer,
            schema,
            source_id_field,
            log_field,
            // Initialize other fields as needed
        })
    }

    pub fn add_logs(&mut self, source_id: i32, logs: Vec<LogJson>) -> Result<(), Box<dyn Error>> {
        let mut count = 0;
        for log in logs {
            let doc = self.log_to_document(source_id, log);
            self.writer.add_document(doc)?;
            count += 1;
        }
        println!("addlogs : {}", count);
        self.writer.commit()?;
        Ok(())
    }

    pub fn search_logs(&self, source_id: i32, page: usize, page_size: usize) -> Result<(Vec<serde_json::Map<std::string::String, serde_json::Value>>, usize), Box<dyn Error>> {
        let reader = self.index.reader()?;
        let searcher = reader.searcher();
    
        let query_parser = QueryParser::for_index(&self.index, vec![self.source_id_field, self.log_field]);
        let user_query = query_parser.parse_query(&format!("source_id:\"{}\"", source_id))?;
        
        
        let offset = (page - 1) * page_size;
        let limit = page_size;
        
        let mut results: Vec<serde_json::Map<std::string::String, serde_json::Value>> = vec![];
        let total_count = searcher.search(&user_query, &Count).unwrap();
        println!("logs total_count {}", total_count);
        println!("searching logs");
        let top_docs = TopDocs::with_limit(limit).and_offset(offset);
        let search_results = searcher.search(&user_query, &(top_docs))?;

        for (_score, doc_address) in search_results {
            if let Some(_value) = searcher.doc(doc_address)?.get_first(self.log_field) {
                match _value.as_json() {
                    Some(v) => {
                        results.push(v.clone());
                        let retrieved_doc = searcher.doc(doc_address)?;
                        println!("{}", self.schema.to_json(&retrieved_doc));
                    },
                    _ => (),
                }
            }
        }

        Ok((results, total_count))
    }

    pub fn log_to_document(&self, source_id: i32, log: LogJson) -> Document {
        let mut doc = Document::new();
        doc.add_i64(self.source_id_field, source_id.into());
        doc.add_json_object(self.log_field, serde_json::from_str(&log.to_text()).unwrap());
        // Add other fields to the document as needed
        doc
    }
}
