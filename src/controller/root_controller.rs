use actix_web::{get, web, HttpRequest, HttpResponse, Responder};

pub fn root_route_config(cfg: &mut web::ServiceConfig) {
    cfg.service(hello_home);
    cfg.service(health_handler);
}

#[utoipa::path(
    context_path = "/api",
    responses(
        (status = 200, description = "Get Hello home controller!"),
        (status = 401, description = "Invalid")
    )
)]
#[get("")]
pub async fn hello_home(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("<h1>Hello actor stock api!</h1>")
}

#[utoipa::path(
    context_path = "/api",
    responses(
        (status = 200, description = "Health check!"),
        (status = 401, description = "Invalid")
    )
)]
#[get("/health")]
pub async fn health_handler(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("Healthy")
}
