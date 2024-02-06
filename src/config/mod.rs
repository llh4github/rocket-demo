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
    name: String,
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
        }
    }
}
