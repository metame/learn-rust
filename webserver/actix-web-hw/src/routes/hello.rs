use actix_web::{get, post, web, HttpResponse, Responder, Result, Scope};

use crate::db;
use crate::types::{GreetName, State};
use crate::util::base64;

#[get("")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("")]
async fn hello_post(data: web::Json<GreetName>) -> Result<String> {
    Ok(format!("{} {}!", data.greeting, data.name))
}

#[get("/{name}")]
async fn hello_name(state: web::Data<State>, path: web::Path<String>) -> impl Responder {
    let name = path.into_inner();
    let conn = state.conn.lock().unwrap();
    db::store_name(&conn, &name);
    HttpResponse::Ok().body(format!("Hello {name}!"))
}

#[get("/{greeting}/{name}")]
async fn custom_greeting(path_data: web::Path<GreetName>) -> Result<String> {
    Ok(format!("{} {}!", path_data.greeting, path_data.name))
}

#[get("/encoded/{encoded_string}")]
async fn encoded_greeting(path: web::Path<String>) -> impl Responder {
    let decoded = String::from_utf8(base64::decode(&path.into_inner()).unwrap()).unwrap();
    web::Redirect::to(format!("/hello/{decoded}"))
}

pub fn service() -> Scope {
    web::scope("/hello")
        .service(hello)
        .service(hello_post)
        .service(encoded_greeting)
        .service(hello_name)
        .service(custom_greeting)
}
