use std::convert::warp::Rejection;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::Filter;

mod api;
mod app;

pub use app::AppState;

fn json_response(v: serde_json::Value) -> Box<dyn warp::Reply> {
    Box::new(warp::reply::json(&v))
}

type AppRef = Arc<Mutex<AppState>>;
type ApiResult = Result<Box<dyn warp::Reply>, warp::Rejection>;

pub async fn run() {
    let state: AppRef = Arc::new(Mutex::new(app::AppState::new().await));

    let state_filter = warp::any().map(move || state.clone());

    let api_prefix = warp::path("api");

    // GET /api/metrics
    let metrics = api_prefix
        .and(warp::path("metrics"))
        .and(warp::get())
        .and(state_filter.clone())
        .and_then(|s: AppRef| async move { api::get_metrics(s).await });

    // GET /api/tweaks
    let tweaks = api_prefix
        .and(warp::path("tweaks"))
        .and(warp::get())
        .and(state_filter.clone())
        .and_then(|s: AppRef| async move { api::list_tweaks(s).await });

    // POST /api/tweaks/apply
    let apply = api_prefix
        .and(warp::path("tweaks").and(warp::path("apply")))
        .and(warp::post())
        .and(warp::body::json())
        .and(state_filter.clone())
        .and_then(|body, s: AppRef| async move { api::apply_tweak(body, s).await });

    // POST /api/tweaks/revert
    let revert = api_prefix
        .and(warp::path("tweaks").and(warp::path("revert")))
        .and(warp::post())
        .and(warp::body::json())
        .and(state_filter.clone())
        .and_then(|body, s: AppRef| async move { api::revert_tweak(body, s).await });

    // GET /api/services
    let services = api_prefix
        .and(warp::path("services"))
        .and(warp::get())
        .and(state_filter.clone())
        .and_then(|s: AppRef| async move { api::list_services(s).await });

    // POST /api/services/stop
    let svc_stop = api_prefix
        .and(warp::path("services").and(warp::path("stop")))
        .and(warp::post())
        .and(warp::body::json())
        .and(state_filter.clone())
        .and_then(|body, s: AppRef| async move { api::stop_service(body, s).await });

    // POST /api/services/disable
    let svc_disable = api_prefix
        .and(warp::path("services").and(warp::path("disable")))
        .and(warp::post())
        .and(warp::body::json())
        .and(state_filter.clone())
        .and_then(|body, s: AppRef| async move { api::disable_service(body, s).await });

    // GET /api/cleaner/scan
    let scan = api_prefix
        .and(warp::path("cleaner").and(warp::path("scan")))
        .and(warp::get())
        .and(state_filter.clone())
        .and_then(|s: AppRef| async move { api::scan_cleaner(s).await });

    // POST /api/cleaner/clean
    let clean = api_prefix
        .and(warp::path("cleaner").and(warp::path("clean")))
        .and(warp::post())
        .and(warp::body::json())
        .and(state_filter.clone())
        .and_then(|body, s: AppRef| async move { api::run_cleaner(body, s).await });

    // GET /api/debloat/list
    let bloat = api_prefix
        .and(warp::path("debloat").and(warp::path("list")))
        .and(warp::get())
        .and(state_filter.clone())
        .and_then(|s: AppRef| async move { api::list_bloatware(s).await });

    // POST /api/debloat/remove
    let rm_bloat = api_prefix
        .and(warp::path("debloat").and(warp::path("remove")))
        .and(warp::post())
        .and(warp::body::json())
        .and(state_filter.clone())
        .and_then(|body, s: AppRef| async move { api::remove_bloatware(body, s).await });

    // GET /api/software
    let software = api_prefix
        .and(warp::path("software"))
        .and(warp::get())
        .and(state_filter.clone())
        .and_then(|s: AppRef| async move { api::list_software(s).await });

    // GET /api/snapshots
    let snapshots = api_prefix
        .and(warp::path("snapshots"))
        .and(warp::get())
        .and(state_filter.clone())
        .and_then(|s: AppRef| async move { api::list_snapshots(s).await });

    // GET /api/audit
    let audit = api_prefix
        .and(warp::path("audit"))
        .and(warp::get())
        .and(state_filter.clone())
        .and_then(|s: AppRef| async move { api::get_audit(s).await });

    let api = metrics
        .or(tweaks).or(apply).or(revert)
        .or(services).or(svc_stop).or(svc_disable)
        .or(scan).or(clean)
        .or(bloat).or(rm_bloat)
        .or(software)
        .or(snapshots)
        .or(audit)
        .unify();

    let index = warp::path::end().and(warp::fs::file("server/static/index.html"));
    let favicon = warp::path("favicon.ico").map(|| "");

    let routes = api.or(index).or(favicon);

    println!("ZingerBoost running at http://127.0.0.1:19999");
    warp::serve(routes).run(([127, 0, 0, 1], 19999)).await;
}
