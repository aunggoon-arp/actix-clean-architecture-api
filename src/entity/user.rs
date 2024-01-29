use chrono::{NaiveDate, NaiveDateTime};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow, ToSchema)]
pub struct User {
    #[serde(skip_serializing)]
    pub id: i32,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub firstname: String,
    pub lastname: String,
    pub profile_image: String,
    pub role_id: i32,
    pub address_id: Option<i32>,
    pub birth_date: Option<NaiveDate>,
    pub phone_no: Option<String>,
    pub description: Option<String>,
    pub height_cm: Option<i32>,
    pub nationality: Option<String>,
    pub gender_id: Option<i32>,
    pub google_auth_id: Option<String>,
    pub point: i32,
    pub follower: i32,
    pub following: i32,
    pub is_deleted: bool,
    pub is_confirmed: bool,
    pub confirmed_user_id: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

impl User {
    pub const TABLE: &'static str = "user_info";
}
