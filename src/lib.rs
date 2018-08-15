extern crate env_logger;
extern crate ws;

mod web;

pub fn run() -> Result<(), &'static str> {
    env_logger::init();

    match ws::listen("0.0.0.0:3012", |out| {
        // Use our router as the handler to route the new connection
        web::Router::new(out)
    }) {
        Ok(_) => Ok(()),
        Err(_) => Err("Unable to bind to address."),
    }
}
