pub fn global_router(cfg: &mut actix_web::web::ServiceConfig) {
    use super::*;

    // health_controller::router(cfg);
    users_controller::router(cfg);
}