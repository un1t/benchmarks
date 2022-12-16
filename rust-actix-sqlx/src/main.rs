use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use sqlx::postgres::{PgPoolOptions};
use sqlx::{postgres::PgPool};
use serde::Serialize;

#[derive(Serialize, sqlx::FromRow)]
pub struct Word {
    pub id: i32,
    pub title: String,
    pub content: String,
}

#[get("/")]
async fn index(pool: web::Data<PgPool>) -> impl Responder {   
    let words = sqlx::query_as::<_, Word>(
            "SELECT * FROM words LIMIT 100"
        )
        .fetch_all(pool.get_ref()).await.unwrap();

    HttpResponse::Ok().json(words)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().expect(".env not found");
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    println!("Starting server at 8080");

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url).await.unwrap();
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}