use axum::http::header;
use axum::http::StatusCode;
use axum::http::Uri;
use axum::response::{IntoResponse, Response};
use rust_embed::RustEmbed;

pub async fn static_handler(uri: Uri) -> impl IntoResponse {
  let mut path = uri.path().trim_start_matches('/').to_string();

  if path.starts_with("assets/") {
    path = path.replace("assets/", "");
  }

  StaticFile(path)
}

#[derive(RustEmbed)]
#[folder = "assets"]
struct Asset;

pub struct StaticFile<T>(pub T);

impl<T> IntoResponse for StaticFile<T>
where
  T: Into<String>,
{
  fn into_response(self) -> Response {
    let path = self.0.into();

    match Asset::get(path.as_str()) {
      Some(content) => {
        let mime = mime_guess::from_path(path).first_or_octet_stream();
        ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
      }
      None => (StatusCode::NOT_FOUND, "404 Not Found").into_response(),
    }
  }
}
