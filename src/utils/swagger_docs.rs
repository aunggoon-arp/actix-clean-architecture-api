use utoipa::{
    openapi::{
        self,
        security::{Http, HttpAuthScheme, SecurityScheme},
    },
    Modify, OpenApi,
};

use crate::controller::root_controller;
use crate::controller::role_controller;
use crate::controller::user_controller;

#[derive(OpenApi)]
#[openapi(
    paths(
        root_controller::hello_home,
        root_controller::health_handler,
        role_controller::hello_role,
        role_controller::get_role_by_id,
        role_controller::get_role_all,
        role_controller::post_create_role,
        role_controller::put_update_role,
        user_controller::hello_user,
        user_controller::get_user_by_id,
        user_controller::post_user_login,
        user_controller::post_register,
        user_controller::put_update_user
    ),
    components(
        schemas(
            utoipa::TupleUnit
        )
    ),
    info(description = "Actor Stock Api"),
    modifiers(&SecurityAddon)
)]

pub struct ApiDoc;

struct SecurityAddon;
impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "Token",
            SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
        );
    }
}