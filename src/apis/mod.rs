mod user;
use std::sync::{Mutex, OnceLock};

use utoipa::OpenApi;
#[derive(OpenApi)]
#[openapi(info(
    description = "My Api description",
    title = "Rokct Demo Endpoint",
    version = "0.1.0"
))]
struct RootApiDoc;

type RouteList = OnceLock<Mutex<Vec<rocket::Route>>>;
static ROUTES: RouteList = RouteList::new();

/// 获取所有api-doc
pub fn get_all_api_doc() -> utoipa::openapi::OpenApi {
    let mut root = RootApiDoc::openapi();
    root.merge(user::Routes::openapi());
    root
}

fn add_routes(routes: Vec<rocket::Route>) {
    let r = ROUTES.get_or_init(|| Mutex::new(vec![]));
    r.lock().unwrap().extend(routes);
}

/// 获取所有api定义
pub fn get_all_api_define() -> Vec<rocket::Route> {
    add_routes(user::Routes::url_list());
    ROUTES.get().unwrap().lock().unwrap().to_vec()
}
