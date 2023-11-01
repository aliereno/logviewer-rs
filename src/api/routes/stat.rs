use actix_web::{get, web, HttpResponse, Result};

use crate::{model::ArcMutexBackgroundData, api::serializers::StatOut, error::MyError};

pub fn config_stat(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/stat").service(get_stats));
}

#[get("")]
pub async fn get_stats(
    background_data: web::Data<ArcMutexBackgroundData>,
) -> Result<HttpResponse, MyError> {
    let data = background_data.lock().map_err(|_| {
        MyError::InternalError
    })?;
    let stats = data.stats.read().unwrap();

    Ok(HttpResponse::Ok().json(StatOut {
        queue_count: stats.queue_count,
        ram_usage: stats.ram_usage,
    }))
}
