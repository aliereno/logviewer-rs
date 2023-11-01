
use crate::model::IndexFields;
use crate::model::LogIndexer;
use crate::model::IndexWriter;

use serde_json::json;
use std::error::Error;
use std::sync::Arc;
use std::sync::RwLock;


use tantivy::collector::Count;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::Index;
use tantivy::IndexSettings;
use tantivy::IndexSortByField;
use tantivy::Order;

impl LogIndexer {
    pub fn new(index_dir: &str) -> Result<Self, Box<dyn Error>> {
        // Create the schema for your log index
        let mut schema_builder = SchemaBuilder::new();
        let id_field = schema_builder.add_text_field("id", STRING | STORED);
        let source_id_field = schema_builder.add_i64_field("source_id", INDEXED | FAST);
        let order_field = schema_builder.add_i64_field("order", INDEXED | FAST);
        let log_text_field = schema_builder.add_text_field("text", TEXT | STORED);
        let log_json_field = schema_builder.add_json_field("log_json", TEXT | STORED);
        // Add other fields to the schema as needed
        let schema = schema_builder.build();

        let settings = IndexSettings {
            sort_by_field: Some(IndexSortByField {
                field: "order".to_string(),
                order: Order::Desc,
            }),
            ..Default::default()
        };

        let index_builder = Index::builder().settings(settings).schema(schema.clone());

        // Create or open the index
        //let index = Index::create_from_tempdir(schema.clone())?;
        let index = match index_builder.create_in_dir(index_dir) {
            Ok(i) => i,
            Err(_) => Index::open_in_dir(index_dir)?,
        };

        Ok(LogIndexer {
            index,
            schema,
            fields: IndexFields{
                id_field,
                source_id_field,
                order_field,
                log_text_field,
                log_json_field,
            }
        })
    }

    pub fn create_writer(&self) -> Result<IndexWriter, Box<dyn Error>> {
        // Create a writer for adding documents to the index
        let writer = Arc::new(RwLock::new(self.index.writer(2_000_000_000)?));

        Ok(IndexWriter {writer, fields: self.fields.clone()})
    }

    pub fn search_logs(
        &self,
        source_id: i32,
        page: usize,
        page_size: usize,
        search: Option<String>,
    ) -> Result<(Vec<serde_json::Value>, usize), Box<dyn Error>> {
        let searcher = self.index.reader()?.searcher();

        let mut search_query = format!("source_id:\"{}\"", source_id);

        match search {
            Some(s) => search_query.push_str(&format!(" AND {}", s)),
            None => ()
        }
        

        let query_parser = QueryParser::for_index(
            &self.index,
            vec![
                self.fields.id_field,
                self.fields.source_id_field,
                self.fields.log_json_field,
                self.fields.log_text_field,
            ],
        );
        let user_query = query_parser.parse_query(&search_query)?;

        let offset = (page - 1) * page_size;
        let limit = page_size;

        let mut results: Vec<serde_json::Value> = vec![];
        let total_count = searcher.search(&user_query, &Count).unwrap();

        let top_docs = TopDocs::with_limit(limit).and_offset(offset);

        let search_results = searcher.search(&user_query, &(top_docs))?;

        for (_score, doc_address) in search_results {
            let _searcher = searcher.doc(doc_address)?;
            let id = _searcher.get_first(self.fields.id_field);
            let log_message = _searcher.get_first(self.fields.log_text_field);
            let log_json: Vec<&Value> = _searcher.get_all(self.fields.log_json_field).collect();

            if log_message.is_some() && id.is_some() {
                match (log_message.unwrap().as_text(), id.unwrap().as_text()) {
                    (Some(l), Some(i)) => {
                        results.push(json!({
                            "id": i,
                            "message": self._log_replace_json(l.to_string(), log_json),
                        }));
                    }
                    (_, _) => (),
                }
            }
        }

        Ok((results, total_count))
    }

    pub fn _log_replace_json(&self, log: String, json_vec: Vec<&Value>) -> String {
        let mut result = log;
        for item in json_vec {
            match item.as_json() {
                Some(js) => {
                    result = result.replacen(
                        "{{JSON}}",
                        &serde_json::to_string(js).unwrap_or_default(),
                        1,
                    )
                },
                None => ()
            }
        }

        result
    }

}

impl IndexWriter {

    pub fn delete_all_indexes(&mut self) -> Result<(), Box<dyn Error>> {

        let mut index_writer_wlock = self.writer.write().unwrap();
        index_writer_wlock.delete_all_documents().unwrap();
        index_writer_wlock.commit()?;

        Ok(())
    }

    pub fn delete_indexes_by_source_id(&mut self, source_id: i32) -> Result<(), Box<dyn Error>> {

        let mut index_writer_wlock = self.writer.write().unwrap();
        let source_id_term = Term::from_field_i64(self.fields.source_id_field, source_id.into());
        index_writer_wlock.delete_term(source_id_term);
        index_writer_wlock.commit()?;

        Ok(())
    }
}

impl Drop for IndexWriter {
    fn drop(&mut self) {
        // Delete all documents when the instance is dropped
        if let Err(e) = self.delete_all_indexes() {
            eprintln!("Failed to delete documents: {}", e);
        }
    }
}