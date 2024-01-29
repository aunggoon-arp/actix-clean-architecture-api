use sqlx::{MySql, MySqlPool, Pool};

use crate::dto::user::{
    CreateUserData, CreateUserInput, UpdateUserData, UpdateUserInput, UserLoginData, UserLoginInput,
};
use crate::entity::user::User;
use crate::error::{ApiResult, CustomError};
use crate::utils::encryption::hash_password;
use crate::utils::helper::has_data;

pub struct UserService;

impl UserService {
    pub async fn get_user_by_id(id: i32, pool: &MySqlPool) -> ApiResult<User> {
        let result = User::find_user_by_id(id, &pool).await?;
        match result {
            Some(data) => Ok(data),
            None => Err(CustomError::NotFoundData),
        }
    }

    pub async fn get_user_login(input: UserLoginInput, pool: &Pool<MySql>) -> ApiResult<User> {
        let pwd_hash = hash_password(input.password.clone()).await;
        let data = UserLoginData {
            email: input.email.clone(),
            password: pwd_hash,
        };
        let result = User::find_user_login(data, &pool).await?;
        match result {
            Some(data) => Ok(data),
            None => Err(CustomError::NotFoundData),
        }
    }

    pub async fn create_user(input: CreateUserInput, pool: &MySqlPool) -> ApiResult<u64> {
        let find_user = User::find_user_by_email(&input.email, &pool).await;
        let user_found: Result<bool, CustomError> = match find_user {
            Ok(user_option) => Ok(has_data(user_option)),
            Err(_err) => Err(_err),
        };
        let pwd_hash = hash_password(input.password.clone()).await;
        match user_found {
            Ok(found) => {
                if found {
                    return Err(CustomError::DuplicateUserEmail);
                }
                let data = CreateUserData {
                    firstname: input.firstname,
                    lastname: input.lastname,
                    email: input.email,
                    password: pwd_hash,
                };
                let result = User::create_user(data, &pool).await;
                match result {
                    Ok(_data) => Ok(_data),
                    Err(err) => Err(err),
                }
            }
            Err(_err) => Err(_err),
        }
    }

    pub async fn update_user(input: UpdateUserInput, pool: &MySqlPool) -> ApiResult<u64> {
        let find_user = User::find_user_by_id(input.id, &pool).await;
        let user_found: Result<bool, CustomError> = match find_user {
            Ok(user_option) => Ok(has_data(user_option)),
            Err(_err) => Err(_err),
        };
        match user_found {
            Ok(found) => {
                if found {
                    return Err(CustomError::DuplicateUserEmail);
                }
                let data = UpdateUserData {
                    id: input.id,
                    firstname: input.firstname,
                    lastname: input.lastname,
                };
                let result = User::update_user(data, &pool).await;
                match result {
                    Ok(data) => Ok(data),
                    Err(err) => Err(err),
                }
            }
            Err(_err) => Err(_err),
        }
    }
}
