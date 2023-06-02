use actix_web::{get, web, Error, HttpResponse, Result};

use crate::{
    api::serializers::PageOut,
    fetcher::fetch_statistics,
    model::{LogEntry, LogJson, Source, Stats},
};

pub fn config_log(cfg: &mut web::ServiceConfig) {
    cfg.service(source_list)
        .service(source_logs)
        .service(source_stats);
}

#[get("/source")]
pub async fn source_list() -> Result<HttpResponse, Error> {
    let source_list = vec![
        Source::new(1, "Source #1".to_string()),
        Source::new(2, "Source #2".to_string()),
    ];

    Ok(HttpResponse::Ok().json(source_list))
}

#[get("/source/{source_id}/logs")]
pub async fn source_logs(source_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(PageOut {
        current_page: 1,
        total_page: 10,
        items: Some(LogJson::get_dummy_vec()),
    }))
}

#[get("/source/{source_id}/stats")]
pub async fn source_stats(source_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(fetch_statistics()))
}
