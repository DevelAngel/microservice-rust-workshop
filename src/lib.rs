#[macro_use]
extern crate serde_derive;

use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use axum::{body::Bytes, Router};

use axum::response::{IntoResponse, Html};
use axum::routing::get;
use axum::extract::Query;

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
}

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
