use std::str::FromStr;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, middleware::Logger};
use serde::Serialize;
use deadpool_postgres::{ManagerConfig, Manager, Pool, RecyclingMethod};
use tokio_postgres::{NoTls, Config};


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
    HttpResponse::Ok().body("OK")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().expect(".env not found");
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("error"));

    let pg_config = Config::from_str(&database_url).unwrap();

    let mgr_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast
    };
    let mgr = Manager::from_config(pg_config, NoTls, mgr_config);
    let pool = Pool::builder(mgr).max_size(50).build().unwrap();

    println!("Starting server at 8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(index)
            .service(ping)

    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}