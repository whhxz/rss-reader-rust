use askama::Template;
use axum::{
    extract,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Json, Router,
};
use serde_json::json;
pub mod model;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello World!" }))
        .route("/json", get(|| async { Json(json!({ "name": "whhxz" })) }))
        .route(
            "/html/:name",
            get(|extract::Path(name): extract::Path<String>| async {
                let t = HelloTemplate { name };
                match t.render() {
                    Ok(html) => Html(html).into_response(),
                    Err(err) => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Failed to render template. Error: {err}"),
                    )
                        .into_response(),
                }
            }),
        );
    let listener = tokio::net::TcpListener::bind("127.0.0.1:6789")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate {
    name: String,
}
