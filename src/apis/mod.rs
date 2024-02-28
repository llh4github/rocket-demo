mod demo1;
mod role;
use std::sync::{Mutex, OnceLock};

use utoipa::OpenApi;

use crate::dto::{IdList, PageQuery};
#[derive(OpenApi)]
#[openapi(
    info(
        description = "My Api description",
        title = "Rokct Demo Endpoint",
        version = "0.1.0"
    ),
    components(schemas(PageQuery, IdList))
)]
struct RootApiDoc;

type RouteList = OnceLock<Mutex<Vec<rocket::Route>>>;
static ROUTES: RouteList = RouteList::new();

/// 获取所有api-doc
pub fn get_all_api_doc() -> utoipa::openapi::OpenApi {
    let mut root = RootApiDoc::openapi();
    root.merge(demo1::Routes::openapi());
    root.merge(role::Routes::openapi());
    root
}

/// 获取所有api定义
pub fn get_all_api_define() -> Vec<rocket::Route> {
    add_routes(demo1::Routes::url_list());
    add_routes(role::Routes::url_list());
    ROUTES.get().unwrap().lock().unwrap().to_vec()
}

fn add_routes(routes: Vec<rocket::Route>) {
    let r = ROUTES.get_or_init(|| Mutex::new(vec![]));
    r.lock().unwrap().extend(routes);
}
