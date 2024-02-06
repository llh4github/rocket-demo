use rocket::serde::json::Json;
use utoipa::OpenApi;

use crate::config::{self, App};

/// utoipa 没法自动生成正确的tag，手动指定下。
const TAG: &str = "测试接口";
#[derive(OpenApi)]
#[openapi(
    tags((name = "测试接口", description = "测试用的")),
    paths(get_config),
    components(
        schemas(config::App)
    ),
)]
pub struct Routes;
impl Routes {
    pub fn url_list() -> Vec<rocket::Route> {
        routes![get_config]
    }
}

/// 获取应用配置
#[utoipa::path(path = "/config", 
    tag = TAG,
    responses((status=200, body=App)))
]
#[get("/config")]
fn get_config() -> Json<App> {
    let cnf: config::App = config::App::figment().extract().unwrap();
    return Json(cnf);
}
