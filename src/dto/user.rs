use utoipa::ToSchema;
use validator::Validate;

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserLoginInput {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6))]
    pub password: String
}


#[derive(Debug, Deserialize)]
pub struct UserLoginData {
    pub email: String,
    pub password: String
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateUserInput {
    #[validate(length(min = 4, max = 10))]
    pub firstname: String,
    #[validate(length(min = 4, max = 10))]
    pub lastname: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6))]
    pub password: String
}

#[derive(Debug, Deserialize)]
pub struct CreateUserData {
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub password: String
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateUserInput {
    pub id: i32,
    #[validate(length(min = 3, max = 10))]
    pub firstname: String,
    #[validate(length(min = 3, max = 10))]
    pub lastname: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserData {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
}
