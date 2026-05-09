use std::sync::Arc;
use tokio::sync::Mutex;
use warp::Filter;

mod api;
mod app;

pub use app::AppState;

pub async fn run() {
    let state = Arc::new(Mutex::new(app::AppState::new().await));

    // API routes
    let api = warp::path("api");

    // CORS
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type"])
        .allow_methods(vec!["GET", "POST"]);

    // Static files
    let static_files = warp::path::end().and(warp::fs::dir("server/static")).or(
        warp::path("static").and(warp::fs::dir("server/static")),
    );

    let routes = api.and(api_routes(state.clone())).with(cors).or(static_files);

    println!("Starting ZingerBoost server on http://127.0.0.1:19999");
    warp::serve(routes).run(([127, 0, 0, 1], 19999)).await;
}

fn api_routes(
    state: Arc<Mutex<app::AppState>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let s = warp::any().map(move || state.clone());

    // GET /api/metrics
    let metrics = warp::path("metrics")
        .and(warp::get())
        .and(s.clone())
        .and_then(api::get_metrics);

    // GET /api/tweaks
    let tweaks = warp::path("tweaks")
        .and(warp::get())
        .and(s.clone())
        .and_then(api::list_tweaks);

    // POST /api/tweaks/apply
    let apply = warp::path("tweaks")
        .and(warp::path("apply"))
        .and(warp::post())
        .and(warp::body::json())
        .and(s.clone())
        .and_then(api::apply_tweak);

    // POST /api/tweaks/revert
    let revert = warp::path("tweaks")
        .and(warp::path("revert"))
        .and(warp::post())
        .and(warp::body::json())
        .and(s.clone())
        .and_then(api::revert_tweak);

    // GET /api/services
    let services = warp::path("services")
        .and(warp::get())
        .and(s.clone())
        .and_then(api::list_services);

    // POST /api/services/stop
    let svc_stop = warp::path("services")
        .and(warp::path("stop"))
        .and(warp::post())
        .and(warp::body::json())
        .and(s.clone())
        .and_then(api::stop_service);

    // POST /api/services/disable
    let svc_disable = warp::path("services")
        .and(warp::path("disable"))
        .and(warp::post())
        .and(warp::body::json())
        .and(s.clone())
        .and_then(api::disable_service);

    // GET /api/cleaner/scan
    let scan = warp::path("cleaner")
        .and(warp::path("scan"))
        .and(warp::get())
        .and(s.clone())
        .and_then(api::scan_cleaner);

    // POST /api/cleaner/clean
    let clean = warp::path("cleaner")
        .and(warp::path("clean"))
        .and(warp::post())
        .and(warp::body::json())
        .and(s.clone())
        .and_then(api::run_cleaner);

    // GET /api/debloat/list
    let bloat = warp::path("debloat")
        .and(warp::path("list"))
        .and(warp::get())
        .and(s.clone())
        .and_then(api::list_bloatware);

    // POST /api/debloat/remove
    let rm_bloat = warp::path("debloat")
        .and(warp::path("remove"))
        .and(warp::post())
        .and(warp::body::json())
        .and(s.clone())
        .and_then(api::remove_bloatware);

    // GET /api/software
    let software = warp::path("software")
        .and(warp::get())
        .and(s.clone())
        .and_then(api::list_software);

    // GET /api/snapshots
    let snapshots = warp::path("snapshots")
        .and(warp::get())
        .and(s.clone())
        .and_then(api::list_snapshots);

    // GET /api/audit
    let audit = warp::path("audit")
        .and(warp::get())
        .and(s.clone())
        .and_then(api::get_audit);

    metrics
        .or(tweaks).or(apply).or(revert)
        .or(services).or(svc_stop).or(svc_disable)
        .or(scan).or(clean)
        .or(bloat).or(rm_bloat)
        .or(software)
        .or(snapshots)
        .or(audit)
}
