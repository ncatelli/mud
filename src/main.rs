extern crate ws;

fn main () {
    if let Err(error) = ws::listen("127.0.0.1:3012", |out| {
        move |msg| {
            println!("Server got message '{}'. ", msg);
            out.send(msg)
        }

    }) {
        println!("Failed to create WebSocket due to {:?}", error);
    }

}
