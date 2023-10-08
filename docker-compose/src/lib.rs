use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::Display;

use bollard::container::ListContainersOptions;
use bollard::service::ContainerSummary;
pub use bollard::Docker;

#[derive(Debug, Deserialize)]
pub enum ProjectState {
  AllUp,
  SomeUp,
  AllDown,
}

impl Display for ProjectState {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        ProjectState::AllUp => "running",
        ProjectState::AllDown => "stopped",
        ProjectState::SomeUp => "issues",
      }
    )
  }
}

#[derive(Debug, Deserialize)]
pub struct Project {
  pub name: String,
  pub path: String,
  pub state: ProjectState,
  pub urls: Vec<String>,
  pub containers: Vec<ContainerSummary>,
}

async fn get_compose_containers(docker: &Docker) -> Vec<ContainerSummary> {
  let mut list_container_filters = HashMap::new();
  list_container_filters.insert("label", vec!["com.docker.compose.project"]);
  docker
    .list_containers(Some(ListContainersOptions {
      all: true,
      filters: list_container_filters,
      ..Default::default()
    }))
    .await
    .unwrap()
}

fn get_urls(containers: &[ContainerSummary]) -> Vec<String> {
  containers
    .iter()
    .filter_map(|c| c.labels.as_ref().unwrap().get("caddy"))
    .cloned()
    .collect()
}

pub async fn get_compose_projects(docker: &Docker) -> Vec<Project> {
  let containers = get_compose_containers(docker).await;
  let c: Vec<((String, String), ContainerSummary)> = containers
    .into_iter()
    .map(|c| {
      let project = c
        .labels
        .as_ref()
        .unwrap()
        .get("com.docker.compose.project")
        .unwrap()
        .clone();
      let project_working_dir = c
        .labels
        .as_ref()
        .unwrap()
        .get("com.docker.compose.project.working_dir")
        .unwrap()
        .clone();
      ((project, project_working_dir), c)
    })
    .collect();
  let mut grouped_containers: HashMap<(String, String), Vec<ContainerSummary>> = HashMap::new();
  for ((name, path), container) in c {
    grouped_containers.entry((name, path)).or_insert(vec![]).push(container);
  }
  let mut v: Vec<_> = grouped_containers
    .into_iter()
    .map(|g| Project {
      name: g.0 .0,
      path: g.0 .1,
      state: match g.1.iter().filter(|c| c.state.as_ref().unwrap() == "running").count() {
        0 => ProjectState::AllDown,
        n if n == g.1.len() => ProjectState::AllUp,
        _ => ProjectState::SomeUp,
      },
      urls: get_urls(&g.1),
      containers: g.1,
    })
    .collect();
  v.sort_by_key(|p| p.name.clone());
  v
}

#[cfg(test)]
mod tests {
  use bollard::Docker;

  use crate::get_compose_projects;
  #[tokio::test]
  async fn test_something() {
    let docker = Docker::connect_with_local_defaults().unwrap();
    let p = get_compose_projects(&docker).await;
    dbg!(p);
  }
}
