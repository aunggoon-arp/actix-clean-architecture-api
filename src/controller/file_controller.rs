use actix_multipart::form::MultipartForm;
use actix_web::{post, web, HttpRequest, HttpResponse, Responder};

use crate::service::file::{FileService, Upload};

pub fn file_route_config(cfg: &mut web::ServiceConfig) {
    cfg.service(hello_file);
    cfg.service(upload_file);
}

#[post("/file")]
async fn hello_file(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("Hello file controller!")
}

#[post("/file/upload")]
async fn upload_file(form: MultipartForm<Upload>) -> HttpResponse {
    FileService::save_file(form).await
}