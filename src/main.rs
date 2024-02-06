#[macro_use]
extern crate rocket;

use std::sync::Mutex;
use std::sync::OnceLock;

use apis::user;
use log::info;
use log4rs;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod apis;

#[derive(OpenApi)]
#[openapi(info(
    description = "My Api description",
    title = "Rokct Demo Endpoint",
    version = "0.1.0"
))]
struct RootApiDoc;

type RouteList = OnceLock<Mutex<Vec<rocket::Route>>>;
static ROUTES: RouteList = RouteList::new();

fn register_api_doc() -> utoipa::openapi::OpenApi {
    let mut root = RootApiDoc::openapi();
    root.merge(user::Routes::openapi());
    root
}

fn add_routes(routes: Vec<rocket::Route>) {
    let r = ROUTES.get_or_init(|| Mutex::new(vec![]));
    r.lock().unwrap().extend(routes);
}
fn register_and_get_url() -> Vec<rocket::Route> {
    add_routes(user::Routes::url_list());
    ROUTES.get().unwrap().lock().unwrap().to_vec()
}

#[launch]
fn rocket() -> _ {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    info!("booting up");
    let builder = rocket::build();
    let routes = register_and_get_url();
    builder
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui/<_..>").url("/api-docs/openapi.json", register_api_doc()),
        )
        .mount("/", routes)
}
