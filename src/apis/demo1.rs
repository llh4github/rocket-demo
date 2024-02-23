use log::debug;
use rocket::routes;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use utoipa::IntoParams;
use utoipa::OpenApi;
use utoipa::ToSchema;
use validator::Validate;

/// utoipa 没法自动生成正确的tag，手动指定下。
const TAG: &str = "接口入参出参案例";
#[derive(OpenApi)]
#[openapi(
    tags((name = "接口入参出参案例", description = "功能测试")),
    paths(add,query),
    components(
        schemas(InputBody,Color)
    ),
)]
pub struct Routes;
impl Routes {
    pub fn url_list() -> Vec<rocket::Route> {
        routes![add, query,]
    }
}
/// 输入参数案例
#[derive(Deserialize, Serialize, Debug, ToSchema, Validate)]
struct InputBody {
    #[validate(range(min = 0, max = 100, message = "年龄不能超过限制"))]
    age: u16,
    #[validate(range(min = 1, max = 10))]
    hight: i32,
}
/// 测试post参数
#[utoipa::path(path = "/demo1/add",
    tag = TAG,
    responses((status=200, body=InputBody)))
]
#[post("/demo1/add", data = "<body>")]
fn add(body: Json<InputBody>) -> Json<InputBody> {
    match body.validate() {
        Ok(_) => (),
        Err(e) => {
            error!("errors : {}", e);
        }
    };
    debug!("input param {:?}", body.0);
    Json(body.0)
}

/// 枚举值想在文档里显示好像只能注册到scheams中了。
#[derive(Debug, PartialEq, ToSchema, FromFormField)]
enum Color {
    Red,
    Blue,
    Green,
}
/// 分页参数（模拟）
#[derive(Debug, PartialEq, FromForm, IntoParams)]
struct Page {
    /// 页大小
    size: usize,
    /// 页码
    index: usize,
}

/// 测试查询参数
#[utoipa::path(path = "/demo1/query",
    tag = TAG,
    responses((status=200)),
    params(
        Page,
        ("name"=&str,Query,description="名字",example="Tom"),
    ),
)]
#[get("/demo1/query?<name>&<age>&<color>&<page..>")]
fn query(name: &str, age: Option<usize>, color: Vec<Color>, page: Page) -> Json<&str> {
    debug!(
        "input name : {} , age {:?} ,color {:?} ,page {:?}",
        name, age, color, page
    );
    Json(name)
}
