#[macro_use]
extern crate rocket;

mod apis;
mod config;
mod entity;
#[launch]
fn rocket() -> _ {
    config::init_server()
}
