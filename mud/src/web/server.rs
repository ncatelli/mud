extern crate ws;

use super::router;

pub fn listen(addr: &str) -> Result<(), &'static str> {
    match ws::listen(&addr, |out| {
        // Use our router as the handler to route the new connection
        router::Router::new(out)
    }) {
        Ok(_) => Ok(()),
        Err(_) => Err("Unable to bind to address."),
    }
}
