mod models;
use crate::models::{Config, GenericResponse};
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};
use awc::{body::BoxBody, error::HeaderValue, http::{Method, header::HeaderName}};

#[actix_web::get("/status")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().json(GenericResponse {
        message: "Hello Client!".into(),
    })
}

async fn route(config: web::Data<Config>, req: HttpRequest) -> impl Responder {
    let ip = match req.peer_addr() {
        Some(s) => s.ip().to_string(),
        None => return HttpResponse::BadRequest().finish(),
    };

    if config.blacklist.contains(&ip) {
        return HttpResponse::Forbidden().finish();
    }

    let uri_string = req.uri().to_string();
    let params: Vec<&str> = uri_string.split("/").collect();
    let service = match params.get(1) {
        Some(s) => s,
        None => return HttpResponse::BadRequest().finish(),
    };

    let mut url = String::new();
    for s in &config.services {
        if s.mapping == *service {
            url = s.url.clone();
            break;
        }
    }

    if url.is_empty() {
        return HttpResponse::NotFound().finish()
    }

    for p in &params[2..] {
        url += "/";
        url += p
    }

    println!("{url}");
    let client = awc::Client::new();
    let raw_response = match req.method() {
        &Method::GET => client.get(url).send().await,
        &Method::POST => client.post(url).send().await,
        &Method::DELETE => client.delete(url).send().await,
        &Method::HEAD => client.head(url).send().await,
        &Method::OPTIONS => client.options(url).send().await,
        &Method::PATCH => client.patch(url).send().await,
        &Method::PUT => client.put(url).send().await,
        _ => return HttpResponse::MethodNotAllowed().finish()
    };
    
    let mut res = match raw_response {
        Ok(res) => res,
        Err(_) => return HttpResponse::BadGateway().finish()
    };

    let body = match res.body().await {
        Ok(b) => b,
        Err(_) => return HttpResponse::InternalServerError().finish()
    };

    let mut forwarded = HttpResponse::new(*&res.status())
        .set_body(BoxBody::new(body));

    for (k, v) in res.headers() {
        forwarded.headers_mut().append(k.clone(), v.clone());
    }

    if config.cors.contains(&ip) {
        forwarded.headers_mut().append(HeaderName::from_static("Access-Control-Allow-Origin"), 
            HeaderValue::from_static("*"));
    }

    forwarded
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config_as_string = std::fs::read_to_string("config.json")?;
    let config: Config = serde_json::from_str(&config_as_string)?;
    let address = format!("{h}:{p}", h = config.host, p = config.port);
    println!("Checkpoint listening on {address}");

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
