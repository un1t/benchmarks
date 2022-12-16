use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::Serialize;
use deadpool_postgres::{Config, Manager, ManagerConfig, Pool, RecyclingMethod, Runtime};
use tokio_postgres::{NoTls, types::ToSql};
use url::{Url, ParseError};


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

    let mut words: Vec<Word> = Vec::new();
    for row in rows {
        let word = Word{
            id:row.get(0),
            title:row.get(1),
            content:row.get(2),
        };
        words.push(word);
    }

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
    pg_config.user = Some(parsed_url.username().to_string());
    pg_config.host = Some(parsed_url.host().unwrap().to_string());
    pg_config.dbname = Some(parsed_url.path()[1..].to_string());
    pg_config.manager = Some(ManagerConfig { recycling_method: RecyclingMethod::Fast });

    let pool = pg_config.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();

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