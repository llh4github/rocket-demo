#[macro_use]
extern crate rocket;
use log::info;
use log4rs;
use rocket::fairing::AdHoc;
use utoipa_swagger_ui::SwaggerUi;

mod apis;
mod config;

#[launch]
fn rocket() -> _ {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    info!("booting up");

    rocket::custom(config::App::figment())
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui/<_..>")
                .url("/api-docs/openapi.json", apis::get_all_api_doc()),
        )
        .mount("/", apis::get_all_api_define())
        .attach(AdHoc::config::<config::App>())
}
