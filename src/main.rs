mod models;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};
use crate::models::{Config, GenericResponse};

#[actix_web::get("/status")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().json(GenericResponse {
        message: "Hello Client!".into()
    })
}

async fn route(config: web::Data<Config>, req: HttpRequest) -> impl Responder {
    let ip = match req.peer_addr() {
        Some(s) => s.ip().to_string(),
        None => return HttpResponse::BadRequest()
    };

    if config.blacklist.contains(&ip) {
        return HttpResponse::Forbidden()
    }

    let uri_string = req.uri().to_string();
    let params: Vec<&str> = uri_string.split("/").collect();
    let service = match params.get(1) {
        Some(s) => s,
        None => return HttpResponse::BadRequest()
    };

    println!("{service}");
    for s in &config.services {
        if s.mapping == *service {
            return HttpResponse::Ok() // Actual implementation will come soon
        }
    }

    HttpResponse::NotFound()
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config_as_string = std::fs::read_to_string("config.json")?;
    let config: Config = serde_json::from_str(&config_as_string)?;
    let address = format!("{h}:{p}", h = config.host, p = config.port);

    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(config.clone()))
        .service(ping)
        .default_service(web::route().to(route))
    })
    .bind(address)?
    .run()
    .await
}
