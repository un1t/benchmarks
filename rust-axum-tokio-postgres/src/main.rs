use axum::{
    extract::State,
    routing::get,
    Router,
    Json, response::IntoResponse,
};
use bb8::{Pool};
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use std::net::SocketAddr;
use serde::Serialize;

type ConnectionPool = Pool<PostgresConnectionManager<NoTls>>;

#[derive(Debug, Serialize)]
pub struct Word {
    pub id: i32,
    pub title: String,
    pub content: String,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect(".env not found");
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let manager =
        PostgresConnectionManager::new_from_stringlike(database_url, NoTls)
            .unwrap();
    let pool = Pool::builder().max_size(10).build(manager).await.unwrap();

    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(index))
        .route("/ping", get(ping))
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index(State(pool): State<ConnectionPool>) ->  impl IntoResponse {
    let client = pool.get().await.unwrap();

    let rows = client.query(
        "SELECT id, title, content FROM words LIMIT 100", &[]
    ).await.unwrap();

    let words: Vec<Word> = rows
        .iter()
        .map(
            |row|
            Word{
                id:row.get(0),
                title:row.get(1),
                content:row.get(2),
            }
        )
        .collect();

    Json(words)
}

async fn ping() -> &'static str {
    "OK"
}
