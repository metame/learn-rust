use actix_web::{get, post, web, HttpResponse, Responder, Result, Scope};
use serde::Deserialize;

use crate::util::base64;

#[derive(Deserialize)]
struct GreetName {
    greeting: String,
    name: String,
}

#[get("")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("")]
async fn hello_post(data: web::Json<GreetName>) -> Result<String> {
    Ok(format!("{} {}!", data.greeting, data.name))
}

#[get("/{name}")]
async fn hello_name(path: web::Path<String>) -> Result<String> {
    let name = path.into_inner();
    Ok(format!("Hello {name}!"))
}

#[get("/{greeting}/{name}")]
async fn custom_greeting(path_data: web::Path<GreetName>) -> Result<String> {
    Ok(format!("{} {}!", path_data.greeting, path_data.name))
}

#[get("/encoded/{encoded_string}")]
async fn encoded_greeting(path: web::Path<String>) -> impl Responder {
    let decoded = String::from_utf8(base64::decode(&path.into_inner()).unwrap()).unwrap();
    web::Redirect::to(format!("/{decoded}"))
}

pub fn service() -> Scope {
    web::scope("/hello")
        .service(hello)
        .service(hello_post)
        .service(encoded_greeting)
        .service(hello_name)
        .service(custom_greeting)
}
