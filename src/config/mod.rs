use std::sync::OnceLock;

use rocket::{fairing::AdHoc, Rocket};
use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseBackend};
use utoipa_swagger_ui::SwaggerUi;

use crate::{apis, config::types::App};

pub mod types;

/// DB实例
static DB: OnceLock<DatabaseBackend> = OnceLock::new();

/// 初始化web服务
pub fn init_server() -> Rocket<rocket::Build> {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    let rk = rocket::custom(App::figment())
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui/<_..>")
                .url("/api-docs/openapi.json", apis::get_all_api_doc()),
        )
        .mount("/", apis::get_all_api_define())
        .attach(AdHoc::config::<App>());
    info!("web server init success!");
    let _ = get_db();
    rk
}
fn get_db() -> &'static DatabaseBackend {
    DB.get_or_init(init_db)
}
/// 初始化数据库连接
fn init_db() -> DatabaseBackend {
    let cnf: App = App::figment().extract().unwrap();

    let url = cnf.db.url;
    let mut options = ConnectOptions::new(url);
    options.min_connections(cnf.db.min_conn);
    options.sqlx_logging_level(cnf.db.log_level);
    let rt = tokio::runtime::Runtime::new().unwrap();
    let db = rt.block_on(async {
        let db = Database::connect(options).await;
        let db = match db {
            Ok(conn) => conn,
            Err(e) => {
                panic!("db connection failed! {}", e)
            }
        };
        db
    });

    let db = db.get_database_backend();
    info!("数据库连接成功！");
    db
}
