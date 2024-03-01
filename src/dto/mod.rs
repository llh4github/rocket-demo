pub mod role;

use std::io::Cursor;

use rocket::{
    http::ContentType,
    response::{self, Responder},
    Request, Response,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Serialize, Deserialize, Validate, ToSchema, Clone)]
pub struct IdList {
    /// 数据ID列表
    #[validate(length(min = 1))]
    pub ids: Vec<i32>,
}

/// 分页参数
#[derive(Serialize, Deserialize, Validate, ToSchema, Clone, Copy)]
pub struct PageQuery {
    /// 当前页
    pub index: u64,
    /// 页大小
    pub size: u64,
}
impl PageQuery {
    pub fn page_index(&self) -> u64 {
        if self.index <= 1 {
            0
        } else {
            self.index
        }
    }
}
impl Default for PageQuery {
    fn default() -> Self {
        Self { index: 1, size: 10 }
    }
}

/// 分页结果
#[derive(Serialize, Deserialize, Validate, ToSchema, Clone)]
pub struct PageResult<T> {
    /// 总页数
    pub tatol_page: u64,
    /// 总条数
    pub totla_item: u64,
    /// 当前页数据
    pub records: Vec<T>,

    /// 当前页
    pub index: u64,

    /// 页大小
    pub size: u64,
}

/// 通用的响应结果
#[derive(Serialize, Deserialize, Validate, ToSchema, Clone)]
pub struct R<T: Serialize> {
    /// 是否成功
    pub seccess: bool,
    /// 消息
    pub msg: String,
    /// 数据
    pub data: T,
}

#[rocket::async_trait]
impl<'r, T: Serialize> Responder<'r, 'static> for R<T> {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let json = serde_json::to_string(&self).unwrap();
        Response::build()
            .header(ContentType::JSON)
            .sized_body(json.len(), Cursor::new(json))
            .ok()
    }
}
