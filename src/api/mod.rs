use actix_web::web;

pub mod routes;


pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
        .service(
            web::scope("/log").configure(routes::log::config_log)
        )
    );
}
