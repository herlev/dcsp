use anyhow::Context;
use axum::{routing::get, Router};
use std::str::FromStr;
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  tracing_subscriber::registry()
    .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "ui=debug".into()))
    .with(tracing_subscriber::fmt::layer())
    .init();
  info!("initializing router...");
  let assets_path = std::env::current_dir().unwrap();
  let api_router = Router::new().route("/status", get(routes::api_status));

  let router = Router::new()
    .nest("/api", api_router)
    .route("/", get(routes::status))
    .nest_service(
      "/assets",
      ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
    )
    .layer(LiveReloadLayer::new());
  let addr = std::net::SocketAddr::from_str("0.0.0.0:8000").unwrap();
  axum::Server::bind(&addr)
    .serve(router.into_make_service())
    .await
    .context("error while starting server")?;
  Ok(())
  // add docker connection to the with_state thing
  // add htmx to local assets
}
