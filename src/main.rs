use actix_cors::Cors;
use actix_web::{
    get, http::header::ContentType, middleware::Logger, post, web, App, HttpResponse, HttpServer,
    Responder,
};
use env_logger::Env;
extern crate redis;
use dotenv::dotenv;
use redis::Commands;

mod model;

use model::SecretRequest;

#[get("/secret/exists/{uuid}")]
async fn does_secret_exist(
    path: web::Path<String>,
    redis: web::Data<redis::Client>,
) -> impl Responder {
    let uuid = path.into_inner();
    let mut connection = redis.get_connection().unwrap();
    let data: Option<String> = connection
        .get(&uuid)
        .or_else(|e| {
            println!("Error: {:?}", e);
            Err(e)
        })
        .unwrap();
    if data.is_some() {
        return HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(format!("{{\"exists\": true}}"));
    }
    HttpResponse::NotFound()
        .content_type(ContentType::json())
        .body(format!("{{\"error\": \"not found\"}}"))
}

#[get("/secret/{uuid}")]
async fn get_and_delete_secret(
    path: web::Path<String>,
    redis: web::Data<redis::Client>,
) -> impl Responder {
    let uuid = path.into_inner();
    let mut connection = redis.get_connection().unwrap();
    let data: Option<String> = connection
        .get(&uuid)
        .or_else(|e| {
            println!("Error: {:?}", e);
            Err(e)
        })
        .unwrap();
    if data.is_some() {
        let _: () = connection
            .del(&uuid)
            .or_else(|e| {
                println!("Error: {:?}", e);
                Err(e)
            })
            .unwrap();
        return HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(format!("{{\"data\": \"{}\"}}", data.unwrap()));
    }
    HttpResponse::NotFound()
        .content_type(ContentType::json())
        .body(format!("{{\"error\": \"not found\"}}"))
}

#[post("/secret")]
async fn create_secret(
    web::Json(secret): web::Json<SecretRequest>,
    redis: web::Data<redis::Client>,
) -> impl Responder {
    let uuid = uuid::Uuid::new_v4();
    let mut connection = redis.get_connection().unwrap();
    let _: () = connection
        .set_ex(uuid.to_string(), secret.data, secret.ttl)
        .or_else(|e| {
            println!("Error: {:?}", e);
            Err(e)
        })
        .unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(format!("{{\"uuid\": \"{}\"}}", uuid))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let db_connection_string =
        std::env::var("redis").expect("redis database connection string must be set");
    let redis = redis::Client::open(db_connection_string).unwrap();
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();
        App::new()
            .app_data(web::Data::new(redis.clone()))
            .wrap(Logger::default())
            .wrap(cors)
            .service(does_secret_exist)
            .service(get_and_delete_secret)
            .service(create_secret)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
