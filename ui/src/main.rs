mod routes;
mod static_assets;

use anyhow::Context;
use axum::{routing::get, Router};
use static_assets::static_handler;
use tower_http::trace::TraceLayer;
use tower_livereload::LiveReloadLayer;
use tracing::{info, Level};
use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  tracing_subscriber::registry()
    .with(tracing_subscriber::fmt::layer())
    .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "ui=debug".into()))
    .with(
      filter::Targets::new()
        .with_target("tower_http::trace::on_response", Level::TRACE)
        .with_target("tower_http::trace::on_request", Level::TRACE)
        .with_target("tower_http::trace::make_span", Level::DEBUG)
        .with_default(Level::INFO),
    )
    .init();
  let api_router = Router::new().route("/status", get(routes::api_status));

  let router = Router::new()
    .route("/", get(routes::status))
    .layer(LiveReloadLayer::new())
    .nest("/api", api_router)
    .route("/assets/*file", get(static_handler))
    .layer(TraceLayer::new_for_http());
  let port = std::env::var("PORT").unwrap_or("8000".into());
  let addr = format!("0.0.0.0:{port}");
  info!("Starting server on {addr}");
  axum::Server::bind(&addr.parse().unwrap())
    .serve(router.into_make_service())
    .await
    .context("error while starting server")?;
  Ok(())
  // add docker connection to the with_state thing
  // add htmx to local assets
}
