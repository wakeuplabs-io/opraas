use axum::response::IntoResponse;

pub async fn health_handler() -> impl IntoResponse {
    (axum::http::StatusCode::OK, "Ok")
}
