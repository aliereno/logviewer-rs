use actix_web::{get, web, Error, HttpResponse, Result};
use serde_json::json;

use crate::{
    api::serializers::{PageFilterIn, PageOut},
    model::ArcMutexBackgroundData, error::MyError,
};

pub fn config_log(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/source").service(source_list).service(source_logs).service(reset_indexes_by_source_id));
}

#[get("")]
pub async fn source_list(
    background_data: web::Data<ArcMutexBackgroundData>,
) -> Result<HttpResponse, MyError> {
    let data = background_data.lock().map_err(|_| {
        MyError::InternalError
    })?;
    let sources = data.sources.clone();

    Ok(HttpResponse::Ok().json(sources))
}

#[get("/{source_id}/logs")]
pub async fn source_logs(
    source_id: web::Path<i32>,
    query: web::Query<PageFilterIn>,
    background_data: web::Data<ArcMutexBackgroundData>,
) -> Result<HttpResponse, MyError> {
    let data = background_data.lock().map_err(|_| {
        MyError::InternalError
    })?;
    let sources = data.sources.clone();

    let source_detail = sources.iter().find(|&s| s.id == *source_id);
    if source_detail.is_none() {
        return Err(MyError::NotFound("source".to_string()));
    }

    let page_size = query.page_size.unwrap_or(20);
    let current_page = query.current_page.unwrap_or(1);
    let search_query = query.search.clone();

    let (items, total_count) = data
        .log_indexer
        .search_logs(source_detail.unwrap().id, current_page, page_size, search_query)
        .unwrap_or_default();

    Ok(HttpResponse::Ok().json(PageOut {
        current_page,
        total_pages: total_count / page_size,
        items: Some(items),
        total_count,
    }))
}

#[get("/{source_id}/reset")]
pub async fn reset_indexes_by_source_id(
    source_id: web::Path<i32>,
    background_data: web::Data<ArcMutexBackgroundData>,
) -> Result<HttpResponse, MyError> {
    let data = background_data.lock().map_err(|_| {
        MyError::InternalError
    })?;
    let sources = data.sources.clone();

    let source_detail = sources.iter().find(|&s| s.id == *source_id);
    if source_detail.is_none() {
        return Err(MyError::NotFound("source".to_string()));
    }
    data.task_manager.send_source_indexing_task(source_detail.unwrap().clone());

    Ok(HttpResponse::Ok().json(json!({"message": "On queue.".to_string()})))
}
