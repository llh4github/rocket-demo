use rocket::serde::json::Json;
use sea_orm::{ActiveModelTrait, IntoActiveModel, Set, TransactionTrait};
use sea_orm_rocket::Connection;
use utoipa::OpenApi;

use crate::{config::Db, dto::role::RoleAddInput, entity::auth_role};
/// utoipa 没法自动生成正确的tag，手动指定下。
const TAG: &str = "角色信息接口";
#[derive(OpenApi)]
#[openapi(
    tags((name = "角色信息接口", description = "操作角色数据的")),
    paths(add),
    components(
        schemas(auth_role::Model,RoleAddInput)
    ),
)]
pub struct Routes;
impl Routes {
    pub fn url_list() -> Vec<rocket::Route> {
        routes![add]
    }
}

/// 添加角色
#[utoipa::path(path = "/role/add", 
    tag = TAG,
    request_body = RoleAddInput,
    responses((status=200, body=auth_role::Model)))
]
#[post("/role/add", data = "<dto>")]
async fn add(conn: Connection<'_, Db>, dto: Json<RoleAddInput>) -> Json<auth_role::Model> {
    let db = conn.into_inner();
    let txn = db.begin().await.unwrap();
    let role = auth_role::ActiveModel {
        name: Set(dto.name.to_owned()),
        code: Set(dto.code.to_owned()),
        updated_by_user_id: Set(2.into()),
        ..Default::default()
    };
    let role = role.insert(&txn).await.expect("insert data err");
    txn.commit().await.unwrap();
    return Json(role);
}
