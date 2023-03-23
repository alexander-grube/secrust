use actix_web::{get, middleware::Logger, web, post, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;
extern crate redis;
use dotenv::dotenv;
use redis::Commands;

mod model;

use model::SecretRequest;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/secret")]
async fn create_secret(
    web::Json(secret): web::Json<SecretRequest>,
    redis: web::Data<redis::Client>,
) -> impl Responder {
        let uuid = uuid::Uuid::new_v4();
        let mut connection = redis.get_connection().unwrap();
        let _: () = connection.set_ex(uuid.to_string(), secret.data, secret.ttl).or_else(|e| {
            println!("Error: {:?}", e);
            Err(e)
        }).unwrap();
        HttpResponse::Ok().body(format!("{{\"uuid\": \"{}\"}}", uuid))
        
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let db_connection_string =
        std::env::var("redis").expect("redis database connection string must be set");
    let redis = redis::Client::open(db_connection_string).unwrap();
    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(redis.clone()))
            .wrap(Logger::default())
            .service(hello)
            .service(create_secret)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
