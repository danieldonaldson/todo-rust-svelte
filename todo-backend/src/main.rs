use axum::{
    routing::{delete, get, post},
    Router,
};
use axum_error::Result;
use sqlx::sqlite::SqlitePool;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

mod api;
mod models;

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenv::dotenv();
    let url = std::env::var("DATABASE_URL")?;
    let pool = SqlitePool::connect(&url).await?;

    let app = Router::new()
        .route("/list", get(api::task::list))
        .route("/new", post(api::task::create))
        .route("/delete/:id", delete(api::task::delete))
        .route("/update/:id", post(api::task::update))
        .route("/task/:id", get(api::task::read))
        .with_state(pool)
        .layer(CorsLayer::very_permissive());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
