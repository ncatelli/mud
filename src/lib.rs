extern crate env_logger;
extern crate ws;

mod web;

pub fn run() -> Result<(), &'static str> {
    env_logger::init();

    match web::listen("0.0.0.0:3012") {
        Ok(_) => Ok(()),
        Err(_) => Err("Unable to bind to address."),
    }
}
