use axum::{
    extract::{Extension, Path, Query},
    routing::{get, post,delete},
    Router,
    http::StatusCode
};
use std::fs;
use once_cell::sync::Lazy;
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod controllers;
mod error;
mod models;
mod utils;

static KEYS: Lazy<models::auth::Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "Your secret here".to_owned());
    models::auth::Keys::new(secret.as_bytes())
});

#[tokio::main]
async fn main() {
    let env = fs::read_to_string(".env").unwrap();
    let (_,database_url) = env.split_once('=').unwrap();
   
   // tracing_subscriber::fmt::init();
    
   tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "axum_api=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
        

        let cors=CorsLayer::new().allow_origin(Any);
        let pool = PgPoolOptions::new()
        .max_connections(50)
        .connect(&database_url)
        .await
        .expect("unable to connect to database");

      
        let app=Router::new()
                .route("/",get(|| async{ "hello world..."}))
                .route("/register",post(controllers::auth::register))
                .route("/product",get(controllers::auth::product_all))
                .route("/product/:id",get(controllers::auth::product_byid))
                .route("/product",post(controllers::auth::product_create))
                .route("/product/:id",delete(controllers::auth::product_delete))
                .route("/login",post(controllers::auth::login))
                .layer(cors)
                .layer(Extension(pool));

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("failed to start server");
}    
