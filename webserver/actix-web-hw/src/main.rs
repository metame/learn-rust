use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::Deserialize;
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

#[get("/hello/{name}")]
async fn hello_name(path: web::Path<String>) -> Result<String> {
    let name = path.into_inner();
    Ok(format!("Hello {name}!"))
}

#[derive(Deserialize)]
struct GreetName {
    greeting: String,
    name: String,
}

#[get("/hello/{greeting}/{name}")]
async fn custom_greeting(path_data: web::Path<GreetName>) -> Result<String> {
    Ok(format!("{} {}!", path_data.greeting, path_data.name))
}

#[post("/hello")]
async fn hello_post(data: web::Json<GreetName>) -> Result<String> {
    Ok(format!("{} {}!", data.greeting, data.name))
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
            .service(hello_name)
            .service(custom_greeting)
            .service(hello_post)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind((config.host, config.port))?
    .run()
    .await
}
