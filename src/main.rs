use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
extern crate redis;
use redis::Commands;
use dotenv::dotenv;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_connection_string = std::env::var("redis").expect("redis database connection string must be set");
    let client = redis::Client::open(db_connection_string).unwrap();
    let mut con = client.get_connection().unwrap();
    let _ : () = con.set("my_key", 42).unwrap();
    let rv: String = con.get("my_key").unwrap();
    println!("Got value: {}", rv);
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}