use axum::{
  http::StatusCode,
  response::IntoResponse,
  routing::{get, post},
  Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::env;

#[tokio::main] // derive a main function
async fn main() {

  // Initialize the logging
  let log_level = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
  env::set_var("RUST_LOG", log_level);
  tracing_subscriber::fmt::init();

  let app = create_app();
  let addr = SocketAddr::from(([127, 0, 0, 1], 3000)); // SocketAddr::from(u8;4,u16)
  tracing::debug!("listening on {}", addr);

  // Serve the app
  axum::Server::bind(&addr)
    .serve(app.into_make_service()) // into_make_service is needed for tower-http
    .await // serve() is a future, so we need to `.await` it
    .unwrap(); // If Err(E) or None, panic

}

fn create_app() -> Router {
  Router::new()
    .route("/", get(root)) // We can set get(get_handler).post(post_handler) etc.
    .route("/users", post(create_user))
}

async fn root() -> &'static str { // 'static lifetime
  "Hello, World!"
}

async fn create_user(
  Json(payload): Json<CreateUser>, // Deserialize the request body
) -> impl IntoResponse {
  let user = User {
    id: 1,
    username: payload.username,
  };
  (StatusCode::CREATED, Json(user)) // Serialize the response body
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct CreateUser {
  username: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct User {
  id: u64,
  username: String,
}

# [cfg(test)]
mod test {
  use super::*;
  use axum::{
    body::Body,
    http::{header, Method, Request},
  };
  use tower::ServiceExt;

  #[tokio::test]
  async fn should_return_hello_world() {
    let req = Request::builder().uri("/").body(Body::empty()).unwrap();
    let res = create_app().oneshot(req).await.unwrap();
    let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
    assert_eq!(bytes, "Hello, World!");
  }

  #[tokio::test]
  async fn should_user_date() {
    let req = Request::builder()
      .uri("/users").method(Method::POST)
      .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
      .body(Body::from(r#"{"username": "jun-uen0"}"#))
      .unwrap();
    let res = create_app().oneshot(req).await.unwrap();
    let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
    let body: String = String::from_utf8(bytes.to_vec()).unwrap();
    let user: User = serde_json::from_str(&body).expect("Failed to deserialize");
    assert_eq!(
      user,
      User {
        id: 1,
        username: "jun-uen0".to_string()
      }
    );
  }
}