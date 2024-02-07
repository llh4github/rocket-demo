use std::net::{IpAddr, Ipv4Addr};

use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment, Profile,
};
use rocket::serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// 应用配置类
#[derive(Debug, Deserialize, ToSchema, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct App {
    /// 应用名称
    pub name: String,
    /// IP address to serve on. **(default: `127.0.0.1`)**
    pub address: IpAddr,
    /// Port to serve on. **(default: `8000`)**
    pub port: u16,
}

impl App {
    pub fn figment() -> Figment {
        Figment::from(rocket::Config::default())
            .merge(Serialized::defaults(App::default()))
            .merge(Toml::file("App.toml").nested())
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
        }
    }
}
