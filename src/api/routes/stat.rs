use actix_web::{get, web, Error, HttpResponse, Result};

use crate::{model::ArcMutexBackgroundData, api::serializers::StatOut};

pub fn config_stat(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/stat").service(get_stats));
}

#[get("")]
pub async fn get_stats(
    background_data: web::Data<ArcMutexBackgroundData>,
) -> Result<HttpResponse, Error> {
    let data = background_data.lock().unwrap();
    let stats = data.stats.read().unwrap();

    Ok(HttpResponse::Ok().json(StatOut {
        queue_count: stats.queue_count,
        ram_usage: stats.ram_usage,
    }))
}
