mod models;
use crate::models::{Config, GenericResponse};
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};
use awc::{
    body::BoxBody,
    error::HeaderValue,
    http::header::HeaderName,
};

#[actix_web::get("/status")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().json(GenericResponse {
        message: "Hello Client!".into(),
    })
}

async fn route(config: web::Data<Config>, req: HttpRequest, b: web::Bytes) -> impl Responder {
    let ip = match req.peer_addr() {
        Some(s) => s.ip().to_string(),
        None => return HttpResponse::BadRequest().json(GenericResponse {
            message: "Your IP could not be read.".into()
        })
    };

    if config.blacklist.contains(&ip) {
        return HttpResponse::Forbidden().json(GenericResponse {
            message: "Not authorized.".into()
        })
    }

    let uri_string = req.uri().to_string();
    let mut params = Vec::new();
    for p in uri_string.split("/") {
        params.push(p.to_owned());
    }

    let service = match params.get(1) {
        Some(s) => s,
        None => return HttpResponse::NotFound().json(GenericResponse {
            message: "This is an unknown service.".into()
        })
    };

    let mut url = match config.services.get(service) {
        Some(url) => url.clone(),
        None => return HttpResponse::NotFound().json(GenericResponse {
            message: "This is an unknown service.".into()
        }),
    };

    for p in &params[2..] {
        url += "/";
        url += p
    }

    let mut serv_req = awc::Client::new().request(req.method().clone(), url);
    for (k, v) in req.headers() {
        serv_req.headers_mut().append(k.clone(), v.clone());
    }

    let mut res = match serv_req.send_body(b).await {
        Ok(res) => res,
        Err(_) => return HttpResponse::BadGateway().json(GenericResponse {
            message: "Service returned an invalid HTTP Response.".into()
        }),
    };

    let body = match res.body().await {
        Ok(b) => b,
        Err(_) => return HttpResponse::InternalServerError().json(GenericResponse {
            message: "Could not decode response body.".into()
        }),
    };

    let mut forwarded = HttpResponse::new(*&res.status()).set_body(BoxBody::new(body));

    for (k, v) in res.headers() {
        forwarded.headers_mut().append(k.clone(), v.clone());
    }

    if config.cors.contains(&ip) {
        forwarded.headers_mut().append(
            HeaderName::from_static("Access-Control-Allow-Origin"),
            HeaderValue::from_static("*"),
        );
    }

    forwarded
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config: Config = serde_json::from_str(&std::fs::read_to_string("config.json")?)?;
    let address = format!("{h}:{p}", h = config.host, p = config.port);
    config.tell_about_self();

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
