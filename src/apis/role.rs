use rocket::serde::json::Json;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    Set, TransactionTrait,
};
use sea_orm_rocket::Connection;
use utoipa::OpenApi;

use crate::{
    config::Db,
    dto::{
        role::{PageParam, RoleAddInput},
        IdList, PageResult,
    },
    entity::auth_role,
};
/// utoipa 没法自动生成正确的tag，手动指定下。
const TAG: &str = "角色信息接口";
#[derive(OpenApi)]
#[openapi(
    tags((name = "角色信息接口", description = "操作角色数据的")),
    paths(add,delete_by_ids,page),
    components(
        schemas(auth_role::Model,
            RoleAddInput,PageResult<auth_role::Model>,
            PageParam)
    ),
)]
pub struct Routes;
impl Routes {
    pub fn url_list() -> Vec<rocket::Route> {
        routes![add, delete_by_ids, page]
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

/// 根据ID删除数据
#[utoipa::path(path = "/role", 
    tag = TAG,
    request_body = IdList,
    responses((status=200, body=auth_role::Model)))
]
#[delete("/role", data = "<dto>")]
async fn delete_by_ids(conn: Connection<'_, Db>, dto: Json<IdList>) -> Json<u64> {
    let db = conn.into_inner();
    let txn = db.begin().await.unwrap();
    let rs = auth_role::Entity::delete_many()
        .filter(auth_role::Column::Id.is_in::<i32, Vec<i32>>(dto.ids.clone()))
        .exec(&txn)
        .await
        .unwrap();
    txn.commit().await.unwrap();
    Json(rs.rows_affected)
}

/// 分页查询
#[utoipa::path(path = "/role/page", 
    tag = TAG,
    request_body = PageParam,
    responses((status=200, body=PageResult<auth_role::Model>)))
]
#[post("/role/page", data = "<dto>")]
async fn page(
    conn: Connection<'_, Db>,
    dto: Json<PageParam>,
) -> Json<PageResult<auth_role::Model>> {
    let db = conn.into_inner();

    let mut condition = Condition::all();
    if dto.name.is_some() {
        let name = dto.name.clone().unwrap();
        condition = condition.add(auth_role::Column::Name.like(name));
    }
    if dto.code.is_some() {
        let tmp = dto.code.clone().unwrap();
        condition = condition.add(auth_role::Column::Code.like(tmp));
    }

    let page = auth_role::Entity::find()
        .order_by_desc(auth_role::Column::Id)
        .filter(condition)
        .paginate(db, dto.page.size);
    let a = page.num_items_and_pages().await.unwrap();
    let rs = page.fetch_page(dto.page.page_index()).await.unwrap();
    let page_rs = PageResult {
        tatol_page: a.number_of_pages,
        totla_item: a.number_of_items,
        records: rs,
    };
    Json(page_rs)
}
