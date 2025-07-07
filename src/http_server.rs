use std::sync::Arc;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use tokio::sync::Mutex;
use crate::ZigService;
use crate::zig_error::{ZigAnyResult, ZigError};
use serde::Deserialize;
use actix_cors::Cors;
use actix_web::http;
use std::env;

#[derive(Deserialize)]
pub struct CreateZigRequest {
    pub user_name: String,
}

struct WebContextContainer {
    zig_service: Arc<ZigService>
}

async fn health() -> impl Responder {
    HttpResponse::Ok().body("<3")
}

async fn create_zig(
    container: web::Data<Mutex<WebContextContainer>>,
    payload: web::Json<CreateZigRequest>,
) -> impl Responder {
    let container = container.lock().await;

    match container.zig_service.dao.create_zig(&payload.user_name) {
        Ok(zig) => HttpResponse::Ok().json(zig),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {}", err)),
    }
}

async fn get_zig(
    container: web::Data<Mutex<WebContextContainer>>,
    path: web::Path<String>,
) -> impl Responder {
    let zig_id_ = path.into_inner();
    let container = container.lock().await;
    match container.zig_service.dao.find_zig_by_id(&zig_id_) {
        Ok(Some(zig)) => HttpResponse::Ok().json(zig),
        Ok(None) => HttpResponse::NotFound().body("Zig not found"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

async fn increment_button_counter(
    container: web::Data<Mutex<WebContextContainer>>,
    path: web::Path<String>,
) -> impl Responder {
    let zig_id_ = path.into_inner();
    let container = container.lock().await;
    match container.zig_service.dao.increase_button_counter(&zig_id_) {
        Ok(Some(zig)) => HttpResponse::Ok().json(zig),
        Ok(None) => HttpResponse::NotFound().body("Zig not found"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

async fn increment_ash_counter(
    container: web::Data<Mutex<WebContextContainer>>,
    path: web::Path<String>,
) -> impl Responder {
    let zig_id_ = path.into_inner();
    let container = container.lock().await;
    match container.zig_service.dao.increase_ash_counter(&zig_id_) {
        Ok(Some(zig)) => HttpResponse::Ok().json(zig),
        Ok(None) => HttpResponse::NotFound().body("Zig not found"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

pub async fn start_http_server(zig_service: Arc<ZigService>) -> ZigAnyResult<()> {
    let container = web::Data::new(Mutex::new(WebContextContainer {
        zig_service: Arc::clone(&zig_service),
    }));

    let host = env::var("HTTP_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("HTTP_PORT").unwrap_or_else(|_| "8000".to_string());
    let bind_addr = format!("{}:{}", host, port);

    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "OPTIONS"])
            .allowed_headers(vec![http::header::CONTENT_TYPE])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(container.clone())
            .route("/health", web::get().to(health))
            .route("/zigs", web::post().to(create_zig))
            .route("/zigs/{id}", web::get().to(get_zig))
            .route("/zigs/{id}/button-increment", web::post().to(increment_button_counter))
            .route("/zigs/{id}/ash-increment", web::post().to(increment_ash_counter))
    })
    .bind(&bind_addr);

    match server {
        Ok(server) => server.run().await.map_err(|err| ZigError::any(&err.to_string())),
        Err(err) => Err(ZigError::any(&err.to_string())),
    }
}