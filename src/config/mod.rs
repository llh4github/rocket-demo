use figment::Figment;
use rocket::{fairing::AdHoc, Rocket};
use sea_orm::ConnectOptions;
use sea_orm_rocket::Database;
use utoipa_swagger_ui::SwaggerUi;

use crate::{apis, config::types::App};
pub mod types;

/// DB实例
// static DB: tokio::sync::OnceCell<DatabaseConnection> = tokio::sync::OnceCell::const_new();

// pub async fn get_db() -> &'static DatabaseConnection {
//     DB.get_or_init(init_db).await
// }
// /// 初始化数据库连接
// async fn init_db() -> DatabaseConnection {
//     let cnf: App = App::figment().extract().unwrap();

//     let url = cnf.db.url;
//     let mut options = ConnectOptions::new(url);
//     options.min_connections(cnf.db.min_conn);
//     options.sqlx_logging_level(cnf.db.log_level);
//     let db = Database::connect(options).await;
//     let db = match db {
//         Ok(conn) => conn,
//         Err(e) => {
//             panic!("db connection failed! {}", e)
//         }
//     };
//     info!("数据库连接成功！");
//     db
// }

#[derive(Database, Debug)]
#[database("sea_orm")]
pub struct Db(SeaOrmPool);

#[derive(Debug, Clone)]
pub struct SeaOrmPool {
    pub conn: sea_orm::DatabaseConnection,
}

#[async_trait]
impl sea_orm_rocket::Pool for SeaOrmPool {
    type Error = sea_orm::DbErr;
    type Connection = sea_orm::DatabaseConnection;

    async fn init(_figment: &Figment) -> Result<Self, Self::Error> {
        let config : App = App::figment().extract().unwrap();
        // let config = figment.extract::<App>().unwrap();
        let mut options: ConnectOptions = ConnectOptions::new(config.db.url);
        options
            // .max_connections(config.max_connections as u32)
            .min_connections(config.db.min_conn)
            // .connect_timeout(Duration::from_secs(config.connect_timeout))
            .sqlx_logging_level(config.db.log_level);
        // if let Some(idle_timeout) = config.idle_timeout {
        // options.idle_timeout(Duration::from_secs(idle_timeout));
        // }
        let conn = sea_orm::Database::connect(options).await?;

        Ok(SeaOrmPool { conn })
    }

    fn borrow(&self) -> &Self::Connection {
        &self.conn
    }
}

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
        .attach(AdHoc::config::<App>())
        .attach(Db::init());
    info!("web server init success!");
    rk
}
