use anyhow::Context;
use askama::Template;
use axum::{
  http::StatusCode,
  response::{Html, IntoResponse, Response},
  routing::get,
  Router,
};
use docker_compose::ProjectState;
use docker_compose::{self, Project};
use serde::Deserialize;
use std::str::FromStr;
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;

use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  tracing_subscriber::registry()
    .with(
      tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "with_axum_htmx_askama=debug".into()),
    )
    .with(tracing_subscriber::fmt::layer())
    .init();
  info!("initializing router...");
  let assets_path = std::env::current_dir().unwrap();
  let router = Router::new()
    .route("/", get(hello))
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
}

async fn hello() -> ProjectsTemplate {
  let docker = docker_compose::Docker::connect_with_local_defaults().unwrap();
  let p = docker_compose::get_compose_projects(&docker).await;
  let template = ProjectsTemplate { projects: p };
  template
}

#[derive(Template, Deserialize, Debug)]
#[template(path = "containers.html")]
struct ProjectsTemplate {
  projects: Vec<Project>,
}
