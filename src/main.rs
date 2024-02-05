#[macro_use]
extern crate rocket;
// use crate::config::ApiDoc;
use log::info;
use log::warn;
use log4rs;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod config;

#[utoipa::path(
    context_path = "",
    responses(
        (status = 200, description = "get test request")
    )
)]
#[get("/hello")]
pub fn hello() -> &'static str {
    info!("info");
    warn!("warn");
    "Hello, world!"
}
#[derive(OpenApi)]
#[openapi(
    paths(::hello,),
    tags((name = "roket-demo", description = "Demo endpoints.")),
)]
struct ApiDoc;
#[launch]
fn rocket() -> _ {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    info!("booting up");
    rocket::build()
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui/<_..>").url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
        .mount("/", routes![hello])
}
