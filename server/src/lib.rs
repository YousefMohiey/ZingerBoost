use std::sync::Arc;
use tokio::sync::Mutex;
use warp::Filter;

mod api;
mod app;

pub use app::AppState;

pub async fn run() {
    let state: Arc<Mutex<AppState>> = Arc::new(Mutex::new(app::AppState::new().await));

    let state_filter = warp::any().map(move || state.clone());

    // Serve index.html at root
    let index = warp::path::end().and(warp::fs::file("server/static/index.html"));

    // All API routes
    let api = warp::path("api").and(
        // Metrics
        warp::get().and(warp::path("metrics")).and(state_filter.clone()).and_then(handle_metrics)
        .or(warp::get().and(warp::path("tweaks")).and(state_filter.clone()).and_then(handle_tweaks))
        .or(warp::post().and(warp::path("tweaks").and(warp::path("apply"))).and(warp::body::json()).and(state_filter.clone()).and_then(handle_apply))
        .or(warp::post().and(warp::path("tweaks").and(warp::path("revert"))).and(warp::body::json()).and(state_filter.clone()).and_then(handle_revert))
        .or(warp::get().and(warp::path("services")).and(state_filter.clone()).and_then(handle_services))
        .or(warp::post().and(warp::path("services").and(warp::path("stop"))).and(warp::body::json()).and(state_filter.clone()).and_then(handle_stop_svc))
        .or(warp::post().and(warp::path("services").and(warp::path("disable"))).and(warp::body::json()).and(state_filter.clone()).and_then(handle_disable_svc))
        .or(warp::get().and(warp::path("cleaner").and(warp::path("scan"))).and(state_filter.clone()).and_then(handle_scan))
        .or(warp::post().and(warp::path("cleaner").and(warp::path("clean"))).and(warp::body::json()).and(state_filter.clone()).and_then(handle_clean))
        .or(warp::get().and(warp::path("debloat").and(warp::path("list"))).and(state_filter.clone()).and_then(handle_bloat))
        .or(warp::post().and(warp::path("debloat").and(warp::path("remove"))).and(warp::body::json()).and(state_filter.clone()).and_then(handle_rm_bloat))
        .or(warp::get().and(warp::path("software")).and(state_filter.clone()).and_then(handle_software))
        .or(warp::get().and(warp::path("snapshots")).and(state_filter.clone()).and_then(handle_snapshots))
        .or(warp::get().and(warp::path("audit")).and(state_filter.clone()).and_then(handle_audit))
        .unify()
    );

    let cors = warp::cors().allow_any_origin();

    println!("ZingerBoost running at http://127.0.0.1:19999");
    warp::serve(api.with(cors).or(index)).run(([127, 0, 0, 1], 19999)).await;
}

// Individual handler functions — each returns warp::reply::Json (same type)
async fn handle_metrics(s: Arc<Mutex<AppState>>) -> Result<warp::reply::Json, warp::Rejection> { api::get_metrics(s).await }
async fn handle_tweaks(s: Arc<Mutex<AppState>>) -> Result<warp::reply::Json, warp::Rejection> { api::list_tweaks(s).await }
async fn handle_apply(b: serde_json::Value, s: Arc<Mutex<AppState>>) -> Result<warp::reply::Json, warp::Rejection> { api::apply_tweak(b, s).await }
async fn handle_revert(b: serde_json::Value, s: Arc<Mutex<AppState>>) -> Result<warp::reply::Json, warp::Rejection> { api::revert_tweak(b, s).await }
async fn handle_services(s: Arc<Mutex<AppState>>) -> Result<warp::reply::Json, warp::Rejection> { api::list_services(s).await }
async fn handle_stop_svc(b: serde_json::Value, s: Arc<Mutex<AppState>>) -> Result<warp::reply::Json, warp::Rejection> { api::stop_service(b, s).await }
async fn handle_disable_svc(b: serde_json::Value, s: Arc<Mutex<AppState>>) -> Result<warp::reply::Json, warp::Rejection> { api::disable_service(b, s).await }
async fn handle_scan(s: Arc<Mutex<AppState>>) -> Result<warp::reply::Json, warp::Rejection> { api::scan_cleaner(s).await }
async fn handle_clean(b: serde_json::Value, s: Arc<Mutex<AppState>>) -> Result<warp::reply::Json, warp::Rejection> { api::run_cleaner(b, s).await }
async fn handle_bloat(s: Arc<Mutex<AppState>>) -> Result<warp::reply::Json, warp::Rejection> { api::list_bloatware(s).await }
async fn handle_rm_bloat(b: serde_json::Value, s: Arc<Mutex<AppState>>) -> Result<warp::reply::Json, warp::Rejection> { api::remove_bloatware(b, s).await }
async fn handle_software(s: Arc<Mutex<AppState>>) -> Result<warp::reply::Json, warp::Rejection> { api::list_software(s).await }
async fn handle_snapshots(s: Arc<Mutex<AppState>>) -> Result<warp::reply::Json, warp::Rejection> { api::list_snapshots(s).await }
async fn handle_audit(s: Arc<Mutex<AppState>>) -> Result<warp::reply::Json, warp::Rejection> { api::get_audit(s).await }
