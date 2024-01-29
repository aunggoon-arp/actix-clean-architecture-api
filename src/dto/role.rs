use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetRoleById {
    pub id: i32,
    pub name: String,
    pub code: String,
}

#[derive(Debug, Deserialize)]
pub struct RequestGetRoleById {
    pub id: i32
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateRoleInput {
    #[validate(length(min = 3))]
    pub name_th: String,
    #[validate(length(min = 3))]
    pub name_en: String,
    #[validate(length(min = 3))]
    pub role_code: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateRoleData {
    pub name_th: String,
    pub name_en: String,
    pub role_code: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateRoleInput {
    pub id: i32,
    #[validate(length(min = 3))]
    pub name_th: String,
    #[validate(length(min = 3))]
    pub name_en: String,
    #[validate(length(min = 3))]
    pub role_code: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateRoleData {
    pub id: i32,
    pub name_th: String,
    pub name_en: String,
    pub role_code: String,
}

