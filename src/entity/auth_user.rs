use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, ToSchema)]
#[sea_orm(table_name = "auth_user")]
#[schema(as = auth_user::Model)]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    pub nickname: String,
    #[sea_orm(unique)]
    pub username: String,
    pub password: String,
    pub created_time: Option<DateTime>,
    pub updated_time: Option<DateTime>,
    pub updated_by_user_id: Option<u32>,
    pub created_by_user_id: Option<u32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}