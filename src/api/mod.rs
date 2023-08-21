use actix_web::web;

pub mod routes;
pub mod serializers;

pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").configure(routes::log::config_log).configure(routes::stat::config_stat));
}
