use actix_files as fs;
use actix_web::{web, App, HttpResponse, HttpServer};
use std::sync::Arc;
use tokio::sync::Mutex;

mod api;
mod app;

pub use app::AppState;

pub async fn run() {
    let state: Arc<Mutex<AppState>> = Arc::new(Mutex::new(app::AppState::new().await));

    let s = actix_web::web::Data::new(state);

    println!("ZingerBoost running at http://127.0.0.1:19999");

    HttpServer::new(move || {
        App::new()
            .app_data(s.clone())
            .route("/api/metrics", web::get().to(api::get_metrics))
            .route("/api/tweaks", web::get().to(api::list_tweaks))
            .route("/api/tweaks/apply", web::post().to(api::apply_tweak))
            .route("/api/tweaks/revert", web::post().to(api::revert_tweak))
            .route("/api/services", web::get().to(api::list_services))
            .route("/api/services/stop", web::post().to(api::stop_service))
            .route(
                "/api/services/disable",
                web::post().to(api::disable_service),
            )
            .route("/api/cleaner/scan", web::get().to(api::scan_cleaner))
            .route("/api/cleaner/clean", web::post().to(api::run_cleaner))
            .route("/api/debloat/list", web::get().to(api::list_bloatware))
            .route("/api/debloat/remove", web::post().to(api::remove_bloatware))
            .route("/api/software", web::get().to(api::list_software))
            .route("/api/snapshots", web::get().to(api::list_snapshots))
            .route("/api/audit", web::get().to(api::get_audit))
            .service(fs::Files::new("/", "server/static").index_file("index.html"))
    })
    .bind("127.0.0.1:19999")
    .expect("Failed to bind")
    .run()
    .await
    .expect("Server error");
}
