use rocket::routes;
use rocket::serde::json::Json;
use utoipa::OpenApi;

#[allow(dead_code)]

/// utoipa 没法自动生成正确的tag，手动指定下。
const TAG: &str = "用户信息接口";
#[derive(OpenApi)]
#[openapi(
    tags((name = "用户信息接口", description = "操作用户数据的")),
    paths(get_list)
)]
pub struct Routes;
impl Routes {
    pub fn url_list() -> Vec<rocket::Route> {
        routes![get_list]
    }
}

/// 获取用户列表
#[utoipa::path(path = "/user/list", tag = TAG)]
#[get("/user/list")]
pub fn get_list() -> Json<&'static str> {
    Json("list")
}
