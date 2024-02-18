use rocket::routes;
use rocket::serde::json::Json;
use sea_orm::EntityTrait;
use utoipa::OpenApi;

use crate::config::get_db;
use crate::entity::user;

/// utoipa 没法自动生成正确的tag，手动指定下。
const TAG: &str = "用户信息接口";
#[derive(OpenApi)]
#[openapi(
    tags((name = "用户信息接口", description = "操作用户数据的")),
    paths(get_by_id),
    components(
        schemas(user::Model)
    ),
)]
pub struct Routes;
impl Routes {
    pub fn url_list() -> Vec<rocket::Route> {
        routes![get_by_id]
    }
}
/// 获取用户列表
#[utoipa::path(path = "/user/id", 
    tag = TAG,
    responses((status=200, body=Option<user::Model>)))
]
#[get("/user/id")]
async fn get_by_id() -> Json<Option<user::Model>> {
    type User = crate::entity::prelude::User;
    let rs = User::find_by_id(1).one(get_db().await).await.unwrap();
    Json(rs)
}

// 获取用户列表
// #[utoipa::path(path = "/user/list",
//     tag = TAG,
//     responses((status=200, body=Vec<User>)))
// ]
// #[get("/user/list")]
// pub fn get_list() -> Json<Vec<User>> {
//     Json(vec![User::new()])
// }
// 添加用户数据
// #[utoipa::path(path = "/user/add",
//     tag = TAG,
//     responses((status=200, body=Option<User>)))
// ]
// #[post("/user/add", data = "<data>")]
// pub fn add(data: Json<User>) -> Json<Option<User>> {
//     info!("接收到的数据： {:?}", data);
//     Json(Some(data.0))
// }
