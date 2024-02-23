use std::net::{IpAddr, Ipv4Addr};

use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment, Profile,
};
use rocket::serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// 应用配置类
#[derive(Debug, Deserialize, ToSchema, Serialize)]
pub struct App {
    /// 应用名称
    pub name: String,
    /// IP address to serve on. **(default: `127.0.0.1`)**
    pub address: IpAddr,
    /// Port to serve on. **(default: `8000`)**
    pub port: u16,
    pub db: Db,
}

impl App {
    pub fn figment() -> Figment {
        Figment::from(rocket::Config::default())
            .merge(Serialized::defaults(App::default()))
            .merge(Toml::file("App.toml").nested())
            .merge(Toml::file("App_local.toml").nested())
            .merge(Toml::file("App_test.toml").nested())
            .merge(Toml::file("App_prod.toml").nested())
            .merge(Env::prefixed("APP_").global())
            .select(Profile::from_env_or("APP_PROFILE", "default"))
    }
}
impl Default for App {
    fn default() -> Self {
        Self {
            name: "Rocket-Web-Api-Demo".to_string(),
            port: 8000,
            address: Ipv4Addr::new(127, 0, 0, 1).into(),
            db: Db::default(),
        }
    }
}

#[derive(Debug, Deserialize, ToSchema, Serialize)]
pub struct Db {
    /// 数据库连接字符串，默认为 **mysql://root:password@localhost:3306/demo**
    /// 密码中含有特殊字符的需要用 url encoder
    pub url: String,
    pub min_conn: u32,
    pub log_level: log::LevelFilter,
}
impl Default for Db {
    fn default() -> Self {
        Self {
            url: "mysql://root:root@localhost:3306/demo".to_string(),
            min_conn: 4,
            log_level: log::LevelFilter::Error,
        }
    }
}
