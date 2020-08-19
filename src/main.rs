mod store;
use serde_json::Value;
use store::State;
use warp::{http, Filter};

#[tokio::main]
async fn main() {
    let store = State::new();
    let store_filter = warp::any().map(move || store.clone());

    let replace_json = warp::post()
        .and(warp::path("json"))
        .and(warp::path::end())
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and(store_filter.clone())
        .and_then(replace_json);

    let get_json = warp::get()
        .and(warp::path("json"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(get_json);

    let routes = get_json.or(replace_json);

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}

async fn replace_json(new_json: Value, store: State) -> Result<impl warp::Reply, warp::Rejection> {
    let mut old_json = store.json_value.write();
    *old_json = new_json;
    Ok(warp::reply::with_status(
        "Replaced internal json",
        http::StatusCode::OK,
    ))
}

async fn get_json(store: State) -> Result<impl warp::Reply, warp::Rejection> {
    let response_json = store.json_value.read();
    Ok(warp::reply::json(&*response_json))
}
