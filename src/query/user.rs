use chrono::Local;
use sqlx::MySqlPool;

use crate::{
    dto::user::{CreateUserData, UpdateUserData, UserLoginData},
    entity::user::User,
    error::ApiResult,
};

impl User {
    pub async fn find_user_by_id(id: i32, pool: &MySqlPool) -> ApiResult<Option<User>> {
        let sql = format!("SELECT * FROM {} WHERE id = ? LIMIT 1", User::TABLE);
        Ok(sqlx::query_as(&sql).bind(id).fetch_optional(pool).await?)
    }

    pub async fn find_user_login(data: UserLoginData, pool: &MySqlPool) -> ApiResult<Option<User>> {
        let sql = format!(
            "SELECT * FROM {} WHERE email = ? AND password_hash = ? LIMIT 1",
            User::TABLE
        );
        Ok(sqlx::query_as(&sql)
            .bind(data.email)
            .bind(data.password)
            .fetch_optional(pool)
            .await?)
    }

    pub async fn find_user_by_email(email: &str, pool: &MySqlPool) -> ApiResult<Option<User>> {
        let sql = format!("SELECT * FROM {} WHERE email = ? LIMIT 1", User::TABLE);
        Ok(sqlx::query_as(&sql)
            .bind(email)
            .fetch_optional(pool)
            .await?)
    }

    pub async fn create_user(data: CreateUserData, pool: &MySqlPool) -> ApiResult<u64> {
        let sql = format!(
            "
            INSERT INTO {} (firstname, lastname, role_id, profile_image, email, password_hash, point, follower, following, is_deleted, is_confirmed, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            ",
            User::TABLE
        );
        let excutor = sqlx::query(&sql)
            .bind(data.firstname)
            .bind(data.lastname)
            .bind(1)
            .bind("no_profile.png")
            .bind(data.email)
            .bind(data.password)
            .bind(0)
            .bind(0)
            .bind(0)
            .bind(false)
            .bind(false)
            .bind(Local::now())
            .execute(pool)
            .await?;
        Ok(excutor.rows_affected())
    }

    pub async fn update_user(data: UpdateUserData, pool: &MySqlPool) -> ApiResult<u64> {
        let sql = format!(
            "
            UPDATE {}
            SET firstname = ?, lastname = ?, updated_at = ?
            WHERE id = ?
            ",
            User::TABLE
        );
        let excutor = sqlx::query(&sql)
            .bind(data.firstname)
            .bind(data.lastname)
            .bind(Local::now())
            .bind(data.id)
            .execute(pool)
            .await?;
        Ok(excutor.rows_affected())
    }
}
