use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use rusqlite::Connection;
use std::sync::Mutex;

mod config;
mod db;
mod routes;
mod types;
mod util;

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let conn = Connection::open_in_memory().expect("Couldn't open in memory connection");
    db::ensure_table(&conn);

    let state = types::State {
        conn: Mutex::new(conn),
    };
    let data = web::Data::new(state);

    let config = config::config();

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(routes::hello::service())
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind((config.host, config.port))?
    .run()
    .await
}
