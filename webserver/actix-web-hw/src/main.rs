use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};

mod config;
mod routes;
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
    let config = config::config();
    HttpServer::new(|| {
        App::new()
            .service(routes::hello::service())
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind((config.host, config.port))?
    .run()
    .await
}
