use rocket::routes;
use rocket::serde::json::Json;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use utoipa::OpenApi;

use crate::config::get_db;
use crate::entity::user;

/// utoipa 没法自动生成正确的tag，手动指定下。
const TAG: &str = "用户信息接口";
#[derive(OpenApi)]
#[openapi(
    tags((name = "用户信息接口", description = "操作用户数据的")),
    paths(get_by_id,search_list),
    components(
        schemas(user::Model)
    ),
)]
pub struct Routes;
impl Routes {
    pub fn url_list() -> Vec<rocket::Route> {
        routes![get_by_id, search_list]
    }
}

type User = crate::entity::prelude::User;
/// 获取用户列表
#[utoipa::path(path = "/user/id", 
    tag = TAG,
    responses((status=200, body=Option<user::Model>)))
]
#[get("/user/id")]
async fn get_by_id() -> Json<Option<user::Model>> {
    let rs = User::find_by_id(1).one(get_db().await).await.unwrap();
    Json(rs)
}

/// 搜索用户列表
#[utoipa::path(path = "/user/search",
    tag = TAG,
    responses((status=200, body=Vec<user::Model>)))
]
#[get("/user/search?<name>")]
async fn search_list(name: String) -> Json<Vec<user::Model>> {
    let list: Vec<user::Model> = User::find()
        .filter(crate::entity::user::Column::Name.contains(name))
        .all(get_db().await)
        .await
        .unwrap();
    return Json(list);
}

