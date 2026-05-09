use std::sync::Arc;
use tokio::sync::Mutex;
use warp::Filter;

mod api;
mod app;

pub use app::AppState;

type AppRef = Arc<Mutex<AppState>>;

pub async fn run() {
    let state: AppRef = Arc::new(Mutex::new(app::AppState::new().await));

    let state_filter = warp::any().map(move || state.clone());

    let routes = warp::any()
        .and(warp::path::full())
        .and(warp::method())
        .and(warp::body::bytes().or(warp::any().map(|| vec![])))
        .unify()
        .and(state_filter)
        .and_then(handle);

    let index = warp::path::end().and(warp::fs::file("server/static/index.html"));

    println!("ZingerBoost running at http://127.0.0.1:19999");
    warp::serve(routes.or(index)).run(([127, 0, 0, 1], 19999)).await;
}

async fn handle(
    path: warp::path::FullPath,
    method: warp::http::Method,
    body: Vec<u8>,
    state: AppRef,
) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let p = path.as_str().to_string();
    let m = method.as_str().to_string();
    let json: serde_json::Value = if body.is_empty() {
        serde_json::Value::Null
    } else {
        serde_json::from_slice(&body).unwrap_or(serde_json::Value::Null)
    };

    let res: warp::reply::Json = match (m.as_str(), p.as_str()) {
        ("GET", "/api/metrics") => api::get_metrics(state).await?,
        ("GET", "/api/tweaks") => api::list_tweaks(state).await?,
        ("POST", "/api/tweaks/apply") => api::apply_tweak(json, state).await?,
        ("POST", "/api/tweaks/revert") => api::revert_tweak(json, state).await?,
        ("GET", "/api/services") => api::list_services(state).await?,
        ("POST", "/api/services/stop") => api::stop_service(json, state).await?,
        ("POST", "/api/services/disable") => api::disable_service(json, state).await?,
        ("GET", "/api/cleaner/scan") => api::scan_cleaner(state).await?,
        ("POST", "/api/cleaner/clean") => api::run_cleaner(json, state).await?,
        ("GET", "/api/debloat/list") => api::list_bloatware(state).await?,
        ("POST", "/api/debloat/remove") => api::remove_bloatware(json, state).await?,
        ("GET", "/api/software") => api::list_software(state).await?,
        ("GET", "/api/snapshots") => api::list_snapshots(state).await?,
        ("GET", "/api/audit") => api::get_audit(state).await?,
        _ => return Ok(Box::new(warp::http::Response::builder().status(404).body("Not found".to_string()).unwrap())),
    };
    Ok(Box::new(res))
}
