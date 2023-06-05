use actix_web::{get, web, Error, HttpResponse, Result};

use crate::{
    api::serializers::{PageOut},
    model::ArcMutexBackgroundData,
};

pub fn config_log(cfg: &mut web::ServiceConfig) {
    cfg.service(source_list)
        .service(source_logs);
}

#[get("/source")]
pub async fn source_list(background_data: web::Data<ArcMutexBackgroundData>) -> Result<HttpResponse, Error> {
    let data = background_data.lock().unwrap();
    let sources = data.sources.clone();

    Ok(HttpResponse::Ok().json(sources))
}

#[get("/source/{source_id}/logs")]
pub async fn source_logs(source_id: web::Path<i32>, background_data: web::Data<ArcMutexBackgroundData>) -> Result<HttpResponse, Error> {

    let data = background_data.lock().unwrap();
    let sources = data.sources.clone();

    let source_detail = sources.iter().find(|&s| s.id == *source_id).unwrap();


    Ok(HttpResponse::Ok().json(PageOut {
        current_page: 1,
        total_page: 10,
        items: Some(data.log_indexer.search_logs(source_detail.id, 1, 10).unwrap_or_default()),
    }))
}
