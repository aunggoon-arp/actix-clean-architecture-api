use chrono::Local;
use sqlx::PgPool;

use crate::{
    error::ApiResult,
    entity::role::Role, dto::role::{CreateRoleData, UpdateRoleData}
};

impl Role {
    pub async fn find_role_by_id(id: i32, pool: &PgPool) -> ApiResult<Role> {
        let sql = format!("SELECT * FROM {} WHERE id = $1", Role::TABLE);
        Ok(sqlx::query_as(&sql).bind(id).fetch_one(pool).await?)
    }

    pub async fn find_role_all(pool: &PgPool) -> ApiResult<Vec<Role>> {
        let sql = format!("SELECT * FROM {}", Role::TABLE);
        Ok(sqlx::query_as(&sql).fetch_all(pool).await?)
    }

    pub async fn create_role(data: CreateRoleData, pool: &PgPool) -> ApiResult<Role> {
        let sql = format!(
            "
            INSERT INTO {} (name_th, name_en, role_code, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            ",
            Role::TABLE
        );
        Ok(sqlx::query_as(&sql)
            .bind(data.name_th)
            .bind(data.name_en)
            .bind(data.role_code)
            .bind(Local::now())
            .bind(Local::now())
            .fetch_one(pool)
            .await?)
    }

    pub async fn update_role(data: UpdateRoleData, pool: &PgPool) -> ApiResult<Role> {
        let sql = format!(
            "
            UPDATE {}
            SET name_th = $1, name_en = $2, role_code = $3)
            RETURNING *
            ",
            Role::TABLE
        );
        Ok(sqlx::query_as(&sql)
            .bind(data.name_th)
            .bind(data.name_en)
            .bind(data.role_code)
            .bind(Local::now())
            .fetch_one(pool)
            .await?)
    }
}
