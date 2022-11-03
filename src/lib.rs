#[macro_use]
extern crate serde_derive;

use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use axum::{body::Bytes, Router};

// lession-1

use axum::response::{IntoResponse, Html};
use axum::routing::get;
use axum::extract::Query;

// lession-2

use axum::extract::{Path, State};
use axum::routing::post;
use axum::http::StatusCode;

use axum::extract::BodyStream;
use futures::StreamExt;

/// Custom type for a shared state
pub type SharedState = Arc<RwLock<AppState>>;
#[derive(Default)]
pub struct AppState {
    db: HashMap<String, Bytes>,
}

pub fn router(state: &SharedState) -> Router<SharedState> {
    Router::with_state(state.clone())
        .route("/", get(hello_world))
        //.route("/hello", get(say_hi_unknown))
        //.route("/hello", get(say_hi_stefan))
        .route("/hello", get(say_hi_stefan_2))
        .route("/kv/:key", get(kv_key_get).post(kv_key_post))
}

// lession-1

async fn hello_world() -> impl IntoResponse {
    "<h1>Hello Axum</h1>"
}

#[allow(dead_code)]
async fn say_hi_unknown() -> impl IntoResponse {
    Html("<h1>Hello Unknown Visitor</h1>")
}

#[allow(dead_code)]
async fn say_hi_stefan(Query(param): Query<HashMap<String, String>>) -> impl IntoResponse {
    let name = if let Some(name) = param.get("name") {
        name
    } else {
        "Unknown Visitor"
    };
    Html(format!("<h1>Hello {name}</h1>"))
}

#[derive(Deserialize)]
struct ParamName {
    name: Option<String>,
}

#[allow(dead_code)]
async fn say_hi_stefan_2(Query(param): Query<ParamName>) -> impl IntoResponse {
    let name = if let Some(name) = param.name {
        name
    } else {
        "Unknown Visitor".to_owned()
    };
    Html(format!("<h1>Hello {name}</h1>"))
}

// lession-2

async fn kv_key_post_simple(Path(key): Path<String>, State(state): State<SharedState>, body: Bytes) {
    let mut state = state.write().unwrap();
    state.db.insert(key, body);
}

async fn kv_key_post(Path(key): Path<String>, State(state): State<SharedState>, mut body: BodyStream) {
    while let Some(chunk) = body.next().await {
        let mut state = state.write().unwrap();
        state.db.insert(key.clone(), chunk.unwrap());
    }

}

async fn kv_key_get(Path(key): Path<String>, State(state): State<SharedState>) -> impl IntoResponse {
    let state = state.read().unwrap();
    if let Some(body) = state.db.get(&key) {
        (StatusCode::OK, body.clone())
    } else {
        (StatusCode::NOT_FOUND, Bytes::new())
    }
}
