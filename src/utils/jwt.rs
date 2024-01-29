use actix_web::HttpRequest;
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};

use crate::{
    config::env::JWT_SECRET,
    entity::user::User,
    error::{ApiResult, CustomError},
};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Claims {
    pub id: i32,
    pub email: String,
    pub role_code: String,
    pub exp: i64,
    pub iat: i64,
}

impl Claims {
    pub fn new(id: i32, email: String, role_code: String) -> Self {
        let iat = Utc::now();
        let exp = iat + Duration::hours(24);
        Self {
            id: id,
            email: email,
            role_code: role_code,
            iat: iat.timestamp(),
            exp: exp.timestamp(),
        }
    }
}

pub fn jwt_sign(input: User, role_code: String) -> ApiResult<String> {
    Ok(jsonwebtoken::encode(
        &Header::default(),
        &Claims::new(input.id, input.email, role_code),
        &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    )?)
}

pub fn jwt_verify(req: HttpRequest) -> Result<Claims, CustomError> {
    let token_access = authorize(req);
    match token_access {
        Ok(token_data) => {
            let token_str = token_data.as_str();
            let token_mut = token_str.replace("Bearer ", "");
            let user = jsonwebtoken::decode::<Claims>(
                token_mut.as_str(),
                &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
                &Validation::default(),
            )
            .map(|data| data.claims)?;
            Ok(user)
        }
        Err(err) => Err(err),
    }
}

fn authorize(req: HttpRequest) -> Result<String, CustomError> {
    if let Some(_jwt) = get_authorized(&req) {
        Ok(String::from(_jwt))
    } else {
        Err(CustomError::WrongCredentials)
    }
}

fn get_authorized<'a>(req: &'a HttpRequest) -> Option<&'a str> {
    req.headers().get("Authorization")?.to_str().ok()
}
