use actix_web::{get, post, put, web, HttpRequest, HttpResponse, Responder};
use serde_json::json;

use crate::{
    dto::{custom::ParamRequest, user::{CreateUserInput, UpdateUserInput, UserLoginInput}},
    service::user::UserService,
    utils::jwt::{jwt_sign, jwt_verify},
    MySqlState,
};

pub fn user_route_config(cfg: &mut web::ServiceConfig) {
    cfg.service(hello_user);
    cfg.service(get_user_by_id);
    cfg.service(post_user_login);
    cfg.service(post_register);
    cfg.service(put_update_user);
}

#[utoipa::path(
    context_path = "/api",
    responses(
        (status = 200, description = "Get Hello user controller!"),
        (status = 401, description = "Invalid")
    )
)]
#[get("/user")]
async fn hello_user() -> impl Responder {
    HttpResponse::Ok().body("Hello user controller!")
}

#[utoipa::path(
    context_path = "/api",
    responses(
        (status = 200, description = "Get user by id!"),
        (status = 401, description = "Invalid")
    ),
    params(
        ParamRequest,
    ),
)]
#[get("/user/getById/{id}")]
async fn get_user_by_id(
    _req: HttpRequest,
    path: web::Path<ParamRequest>,
    db_state: web::Data<MySqlState>,
) -> impl Responder {
    let param = path.into_inner();
    let result = UserService::get_user_by_id(param.id, &db_state.db).await;
    match result {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_err) => {
            let json = json!({"message": _err.to_string()});
            HttpResponse::Unauthorized().json(web::Json(json))
        }
    }
}

#[utoipa::path(
    context_path = "/api",
    responses(
        (status = 200, description = "Login !"),
        (status = 409, description = "Invalid Request Format")
    ),
    request_body(content = String, example = json!({"email": "johndoe@example.com", "password": "password123"})),
)]
#[post("/user/login")]
async fn post_user_login(
    _req: HttpRequest,
    body: web::Json<UserLoginInput>,
    db_state: web::Data<MySqlState>,
) -> impl Responder {
    let input = UserLoginInput {
        email: body.email.clone(),
        password: body.password.clone(),
    };
    let result = UserService::get_user_login(input, &db_state.db).await;
    match result {
        Ok(data) => {
            let sign_token_result = jwt_sign(data, String::from("user"));
            match sign_token_result {
                Ok(token) => {
                    let json = json!({"token": token});
                    HttpResponse::Ok().json(web::Json(json))
                }
                Err(_err) => {
                    let json = json!({"message": "Internal server error."});
                    HttpResponse::Unauthorized().json(web::Json(json))
                }
            }
        }
        Err(_err) => { 
            let json = json!({"message": _err.to_string()});
            HttpResponse::BadRequest().json(web::Json(json)) 
        },
    }
}

#[utoipa::path(
    context_path = "/api",
    responses(
        (status = 200, description = "Create user !"),
        (status = 409, description = "Invalid Request Format")
    ),
    request_body(content = String, example = json!({"email": "johndoe@example.com", "password": "password123", "firstname": "Mr.Sun", "lastname": "Hapoon"})),
)]
#[post("/user/register")]
async fn post_register(
    _req: HttpRequest,
    body: web::Json<CreateUserInput>,
    db_state: web::Data<MySqlState>,
) -> impl Responder {
    let input = CreateUserInput {
        email: body.email.clone(),
        password: body.password.clone(),
        firstname: body.firstname.clone(),
        lastname: body.lastname.clone(),
    };
    let result = UserService::create_user(input, &db_state.db).await;
    match result {
        Ok(_data) => HttpResponse::Created().finish(),
        Err(_err) => {
            let json = json!({"message": _err.to_string()});
            HttpResponse::BadRequest().json(web::Json(json))
        }
    }
}

#[utoipa::path(
    context_path = "/api",
    responses(
        (status = 200, description = "Update user !"),
        (status = 409, description = "Invalid Request Format")
    ),
    request_body(content = String, example = json!({"id": 1, "firstname": "Mr.Sun", "lastname": "Hapoon"})),
)]
#[put("/user/update")]
async fn put_update_user(
    _req: HttpRequest,
    body: web::Json<UpdateUserInput>,
    db_state: web::Data<MySqlState>,
) -> impl Responder {
    let authorize = jwt_verify(_req);
    match authorize {
        Ok(_user_claim) => {
            let input = UpdateUserInput {
                id: body.id.clone(),
                firstname: body.firstname.clone(),
                lastname: body.lastname.clone(),
            };
            let result = UserService::update_user(input, &db_state.db).await;
            match result {
                Ok(_data) => HttpResponse::Ok().finish(),
                Err(_err) => {
                    let json = json!({"message": "Update data error."});
                    HttpResponse::BadRequest().json(web::Json(json))
                }
            }
        }
        Err(_err) => {
            let json = json!({"message": "Unauthorized."});
            HttpResponse::Unauthorized().json(web::Json(json))
        }
    }
}
