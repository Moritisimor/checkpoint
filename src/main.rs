mod models;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use crate::models::{Config, GenericResponse};

#[actix_web::get("/status")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().json(GenericResponse {
        message: "Hello Client!".into()
    })
}

async fn route() -> impl Responder {
    HttpResponse::Ok()
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config_as_string = std::fs::read_to_string("config.json")?;
    let config: Config = serde_json::from_str(&config_as_string)?;

    let mut address = String::new();
    address += config.host.as_str();
    address += ":";
    address += config.port.to_string().as_str();
    println!("{address}");

    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(config.clone()))
        .service(ping)
        .service(web::resource("/").to(route))
    })
    .bind(address)?
    .run()
    .await
}
