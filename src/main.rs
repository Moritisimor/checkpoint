mod models;
use actix_web::{App, HttpResponse, HttpServer, Responder};

use crate::models::GenericResponse;

#[actix_web::get("/")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().json(GenericResponse {
        message: "Hello Client!".into()
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(ping)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
