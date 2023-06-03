use actix_web::{get, web, Error, HttpResponse, Result};

use crate::{
    api::serializers::PageOut,
    fetcher::fetch_statistics,
    model::{LogJson, ArcMutexBackgroundData},
};

pub fn config_log(cfg: &mut web::ServiceConfig) {
    cfg.service(source_list)
        .service(source_logs)
        .service(source_stats);
}

#[get("/source")]
pub async fn source_list(background_data: web::Data<ArcMutexBackgroundData>) -> Result<HttpResponse, Error> {
    let data = background_data.lock().unwrap();
    let result = data.sources.clone();

    Ok(HttpResponse::Ok().json(result))
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
