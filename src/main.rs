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

  let app = Router::new()
    .route("/", get(root)) // We can set get(get_handler).post(post_handler) etc.
    .route("/users",post(create_user));
  let addr = SocketAddr::from(([127, 0, 0, 1], 3000)); // SocketAddr::from(u8;4,u16)
  tracing::debug!("listening on {}", addr);

  // Serve the app
  axum::Server::bind(&addr)
    .serve(app.into_make_service()) // into_make_service is needed for tower-http
    .await // serve() is a future, so we need to `.await` it
    .unwrap(); // If Err(E) or None, panic

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

#[derive(Deserialize)]
struct CreateUser {
  username: String,
}

#[derive(Serialize)]
struct User {
  id: u64,
  username: String,
}