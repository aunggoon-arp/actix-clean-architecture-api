use sqlx::PgPool;

use crate::dto::role::{
    CreateRoleData, CreateRoleInput, GetRoleById, UpdateRoleData, UpdateRoleInput,
};
use crate::entity::role::Role;
use crate::error::ApiResult;

pub struct RoleService;

impl RoleService {
    pub async fn get_role_by_id(id: i32, pool: &PgPool) -> ApiResult<GetRoleById> {
        let role = Role::find_role_by_id(id, &pool).await?;
        Ok(GetRoleById {
            id: role.id,
            code: role.role_code,
            name: role.name_en,
        })
    }

    pub async fn get_role_all(pool: &PgPool) -> ApiResult<Vec<Role>> {
        let roles = Role::find_role_all(&pool).await?;
        Ok(roles)
    }

    pub async fn create_role(input: CreateRoleInput, pool: &PgPool) -> ApiResult<Role> {
        let data = CreateRoleData {
            name_th: input.name_th,
            name_en: input.name_en,
            role_code: input.role_code,
        };
        let role = Role::create_role(data, &pool).await?;
        Ok(role)
    }

    pub async fn update_role(input: UpdateRoleInput, pool: &PgPool) -> ApiResult<Role> {
        let data = UpdateRoleData {
            id: input.id,
            name_th: input.name_th,
            name_en: input.name_en,
            role_code: input.role_code,
        };
        let role = Role::update_role(data, &pool).await?;
        Ok(role)
    }
}
