use std::str::FromStr;
use serde::Serialize;
use deadpool_postgres::{ManagerConfig, Manager, Pool, RecyclingMethod};
use tokio_postgres::{NoTls, Config};
use dotenv::dotenv;
use ntex::web::{self, App, Error, HttpResponse};


#[derive(Debug, Serialize)]
pub struct Word {
    pub id: i32,
    pub title: String,
    pub content: String,
}


async fn index(pool: web::types::State<Pool>) -> Result<HttpResponse, Error> {
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

    Ok(HttpResponse::Ok().json(&words))
}

async fn ping() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("OK"))
}


#[ntex::main]
async fn main() {
    dotenv().ok();
    dotenv::dotenv().expect(".env not found");
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pg_config = Config::from_str(&database_url).unwrap();

    let mgr_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast
    };
    let mgr = Manager::from_config(pg_config, NoTls, mgr_config);
    let pool = Pool::builder(mgr).max_size(10).build().unwrap();

    let server = web::server(move || {
        App::new()
            .state(pool.clone())
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/ping").route(web::get().to(ping)))
    })
    .bind(("127.0.0.1", 8080))
    .unwrap()
    .run();

    server.await.unwrap()
}
