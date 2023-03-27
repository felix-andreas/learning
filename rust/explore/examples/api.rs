/// basic axum usage

use std::net::SocketAddr;

use {
    axum::{
        debug_handler,
        extract::{Path, Query},
        http::StatusCode,
        routing::{get, post},
        Json, Router,
    },
    serde::{Deserialize, Serialize},
    serde_json::{json, Value},
    tower_http::trace::TraceLayer,
    tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt},
};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer().with_target(true))
        .init();

    let app = Router::new()
        .route("/path/:a/:b", get(path))
        .route("/query", post(query))
        .route("/error", post(error))
        .route("/json", post(json))
        .route("/typed-json", post(typed_json))
        .layer(TraceLayer::new_for_http());
    let addr = SocketAddr::from(([127, 0, 0, 1], 3003));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[debug_handler]
async fn path(Path((a, b)): Path<(i32, i32)>) -> Json<Value> {
    println!("path: {a} {b}");
    Json(json!(a + b))
}

async fn json(Json(payload): Json<Value>) -> Json<Value> {
    Json(payload["foo"].clone())
}

async fn typed_json(Json(Foo { number }): Json<Foo>) -> Json<Foo> {
    Json(Foo { number: number + 1 })
}

#[derive(Deserialize, Serialize)]
struct Foo {
    number: i32,
}

#[derive(Deserialize, Serialize)]
struct Pagination {
    page: usize,
    per_page: usize,
}

async fn query(Query(query): Query<Pagination>) -> Json<Pagination> {
    Json(query)
}

async fn error() -> Result<(), (StatusCode, &'static str)> {
    Err((StatusCode::INTERNAL_SERVER_ERROR, "This always errors"))
}
