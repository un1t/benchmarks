use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;
use deadpool_postgres::{ManagerConfig, Manager, Pool, RecyclingMethod, Runtime};
use tokio_postgres::{NoTls, Config};
use url::{Url};


#[derive(Debug, Serialize)]
pub struct Word {
    pub id: i32,
    pub title: String,
    pub content: String,
}

#[get("/")]
async fn index(pool: web::Data<Pool>) -> impl Responder {
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

    HttpResponse::Ok().json(words)
}

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().json("OK")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().expect(".env not found");
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let parsed_url = Url::parse(&database_url).unwrap();

    let mut pg_config = Config::new();
    pg_config.user(parsed_url.username());
    pg_config.host(&parsed_url.host().unwrap().to_string());
    pg_config.dbname(&parsed_url.path()[1..]);

    let mgr_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast
    };
    let mgr = Manager::from_config(pg_config, NoTls, mgr_config);
    let pool = Pool::builder(mgr).max_size(10).build().unwrap();

    println!("Starting server at 8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(index)
            .service(ping)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}