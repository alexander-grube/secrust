use actix_web::{get, middleware::Logger, post, App, HttpResponse, HttpServer, Responder};
extern crate redis;
use dotenv::dotenv;
use redis::Commands;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/secret")]
async fn create_secret(msg: String) -> impl Responder {
    HttpResponse::Ok().body(msg)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_connection_string =
        std::env::var("redis").expect("redis database connection string must be set");
    let client = redis::Client::open(db_connection_string).unwrap();
    let mut con = client.get_connection().unwrap();
    let _: () = con.set("my_key", 42).unwrap();
    let rv: String = con.get("my_key").unwrap();
    println!("Got value: {}", rv);
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(hello)
            .service(create_secret)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
