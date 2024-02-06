use log::info;
use rocket::routes;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use utoipa::OpenApi;
use utoipa::ToSchema;

/// utoipa 没法自动生成正确的tag，手动指定下。
const TAG: &str = "用户信息接口";
#[derive(OpenApi)]
#[openapi(
    tags((name = "用户信息接口", description = "操作用户数据的")),
    paths(get_list,add),
    components(
        schemas(User)
    ),
)]
pub struct Routes;
impl Routes {
    pub fn url_list() -> Vec<rocket::Route> {
        routes![get_list, add]
    }
}

/// 获取用户列表
#[utoipa::path(path = "/user/list", 
    tag = TAG,
    responses((status=200, body=Vec<User>)))
]
#[get("/user/list")]
pub fn get_list() -> Json<Vec<User>> {
    Json(vec![User::new()])
}
/// 添加用户数据
#[utoipa::path(path = "/user/add", 
    tag = TAG,
    responses((status=200, body=Option<User>)))
]
#[post("/user/add", data = "<data>")]
pub fn add(data: Json<User>) -> Json<Option<User>> {
    info!("接收到的数据： {:?}", data);
    Json(Some(data.0))
}
/// 模拟用户数据
///
/// 接入数据库后删除
#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct User {
    /// 名称
    name: String,
    /// 年纪
    #[schema(example = 1)]
    age: i32,
}
impl User {
    fn new() -> User {
        User {
            name: "Tom".to_string(),
            age: 33,
        }
    }
}
