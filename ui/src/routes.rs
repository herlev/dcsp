use askama::Template;
use docker_compose::Project;
use docker_compose::ProjectState;
use serde::Deserialize;

pub async fn status() -> StatusTemplate {
  let docker = docker_compose::Docker::connect_with_local_defaults().unwrap();
  let p = docker_compose::get_compose_projects(&docker).await;
  let template = StatusTemplate { projects: p };
  template
}

pub async fn api_status() -> ApiStatusTemplate {
  let docker = docker_compose::Docker::connect_with_local_defaults().unwrap();
  let p = docker_compose::get_compose_projects(&docker).await;
  let template = ApiStatusTemplate { projects: p };
  template
}

#[derive(Template, Deserialize, Debug)]
#[template(path = "status.html")]
pub struct StatusTemplate {
  projects: Vec<Project>,
}

#[derive(Template, Deserialize, Debug)]
#[template(path = "api/status.html")]
pub struct ApiStatusTemplate {
  projects: Vec<Project>,
}
