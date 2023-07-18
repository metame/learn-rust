use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::env;

type Host = String;
type Port = u16;

struct Config {
    host: Host,
    port: Port,
}

fn config() -> Config {
    Config {
        host: env::var("HOST").unwrap_or(String::from("127.0.0.1")),
        port: env::var("PORT")
            .unwrap_or("8081".to_string())
            .parse::<u16>()
            .unwrap(),
    }
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config();
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind((config.host, config.port))?
    .run()
    .await
}
