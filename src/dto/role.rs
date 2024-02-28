use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use super::PageQuery;

/// 添加角色所需数据
#[derive(Serialize, Deserialize, Validate, ToSchema)]
pub struct RoleAddInput {
    /// 名称
    pub name: String,
    /// 代号
    pub code: String,
}

#[derive(Serialize, Deserialize, Validate, ToSchema)]
pub struct PageParam {
    pub name: Option<String>,
    pub code: Option<String>,
    pub page: PageQuery,
}
