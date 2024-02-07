#!#[cfg(test)]
use log::debug;
use sea_orm::EntityTrait;

use crate::{
    config::{self, get_db},
    entity::user::Entity as User,
};

#[tokio::test]
async fn t1() {
    config::init_server();
    let db = get_db().await;
    let u = User::find_by_id(1).one(db).await.unwrap();
    debug!("find user {:?}", u)
}
