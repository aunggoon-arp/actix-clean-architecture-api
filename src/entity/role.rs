use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Role {
    pub id: i32,
    pub name_th: String,
    pub name_en: String,
    pub role_code: String,
    pub is_deleted: bool,
}

impl Role {
    pub const TABLE: &'static str = "sys_user_role";
}
