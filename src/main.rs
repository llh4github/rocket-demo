#[macro_use]
extern crate rocket;

mod apis;
mod config;
mod entity;
mod test;
#[launch]
fn rocket() -> _ {
    config::init_server()
}
