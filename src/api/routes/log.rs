use actix_web::{get, web, HttpResponse, Result, Error};

use crate::fetcher::{fetch_statistics};

pub fn config_log(cfg: &mut web::ServiceConfig) {
    cfg.service(stats);
}


#[get("/stats")] // TODO: add query filter params
pub async fn stats() -> Result<HttpResponse, Error> {
    let stats = fetch_statistics();
    Ok(HttpResponse::Ok().json(stats))
}
