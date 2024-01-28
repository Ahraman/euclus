use axum::{extract::Path, routing::get, Router};
use dotenvy;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();

    let router = build_router();

    let listener = TcpListener::bind("localhost:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

pub fn build_router() -> Router<()> {
    Router::new()
        .route("/wiki/", get(root))
        .route("/wiki/:page", get(get_page))
}

pub async fn root() -> String {
    "Hello world!".to_owned()
}

pub async fn get_page(Path(page): Path<String>) -> String {
    format!("Hello {page}")
}
