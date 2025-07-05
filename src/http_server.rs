use std::sync::Arc;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use tokio::sync::Mutex;
use crate::ZigService;
use crate::zig_error::{ZigAnyResult, ZigError};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateZigRequest {
    pub user_name: String,
}

struct WebContextContainer {
    zig_service: Arc<ZigService>
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

    let server = HttpServer::new(move || {
        App::new()
            .app_data(container.clone())
            .route("/zigs", web::post().to(create_zig))
            .route("/zigs/{id}", web::get().to(get_zig))
            .route("/zigs/{id}/button-increment", web::post().to(increment_button_counter))
            .route("/zigs/{id}/ash-increment", web::post().to(increment_ash_counter))
    })
    .bind("127.0.0.1:8000");

    match server {
        Ok(server) => match server.run().await {
            Ok(_) => Ok(()),
            Err(err) => Err(ZigError::any(err.to_string().as_str())),
        },
        Err(err) => Err(ZigError::any(err.to_string().as_str())),
    }
}