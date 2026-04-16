use axum::{Json, Router, routing::get};
use serde::Serialize;
use std::net::SocketAddr;

#[derive(Serialize)]
struct MessageResponse {
    message: &'static str,
}

#[derive(Serialize)]
struct MeResponse {
    name: &'static str,
    email: &'static str,
    github: &'static str,
}

fn app() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/me", get(me))
}

async fn root() -> Json<MessageResponse> {
    Json(MessageResponse {
        message: "API is running",
    })
}

async fn health() -> Json<MessageResponse> {
    Json(MessageResponse { message: "healthy" })
}

async fn me() -> Json<MeResponse> {
    Json(MeResponse {
        name: "Gideon Bature",
        email: "infoaboutgideon@gmail.com",
        github: "https://github.com/GideonBature",
    })
}

#[tokio::main]
async fn main() {
    let port = std::env::var("PORT")
        .ok()
        .and_then(|value| value.parse::<u16>().ok())
        .unwrap_or(3000);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind TCP listener");

    axum::serve(listener, app())
        .await
        .expect("server exited unexpectedly");
}

#[cfg(test)]
mod tests {
    use super::app;
    use axum::{
        body::{self, Body},
        http::{Request, StatusCode, header},
    };
    use tower::util::ServiceExt;

    async fn assert_json_response(path: &str, expected_body: &str) {
        let response = app()
            .oneshot(
                Request::builder()
                    .uri(path)
                    .body(Body::empty())
                    .expect("failed to build request"),
            )
            .await
            .expect("request failed");

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get(header::CONTENT_TYPE).unwrap(),
            "application/json"
        );

        let body = body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("failed to read body");
        assert_eq!(
            std::str::from_utf8(&body).expect("body should be utf-8"),
            expected_body
        );
    }

    #[tokio::test]
    async fn root_returns_expected_json() {
        assert_json_response("/", "{\"message\":\"API is running\"}").await;
    }

    #[tokio::test]
    async fn health_returns_expected_json() {
        assert_json_response("/health", "{\"message\":\"healthy\"}").await;
    }

    #[tokio::test]
    async fn me_returns_expected_json() {
        assert_json_response(
            "/me",
            "{\"name\":\"Gideon Bature\",\"email\":\"infoaboutgideon@gmail.com\",\"github\":\"https://github.com/GideonBature\"}",
        )
        .await;
    }
}
