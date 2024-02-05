#[macro_use]
extern crate rocket;
use log::info;
use log::warn;
use log4rs;
#[get("/")]
fn hello() -> &'static str {
    info!("info");
    warn!("warn");
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    info!("booting up");
    rocket::build().mount("/", routes![hello])
}
