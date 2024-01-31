use axum::{extract::Path, routing::get, Router};
use diesel::{
    query_dsl::methods::{FilterDsl, SelectDsl},
    Connection, ExpressionMethods, PgConnection, RunQueryDsl, SelectableHelper,
};
use dotenvy;
use tokio::net::TcpListener;

use euclus::models::Page;

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
    use euclus::schema::pages::dsl::*;

    let conn = &mut establish_connection();
    let res = pages
        .filter(title.eq(&page))
        .select(Page::as_select())
        .first(conn);

    if let Ok(p) = res {
        format!("Hello {}", p.title)
    } else {
        format!("Page '{}' not found!", page)
    }
}

fn establish_connection() -> PgConnection {
    let database_url = std::env::var("DATABASE_URL").unwrap();
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {database_url}"))
}
