extern crate parser;
extern crate ws;

pub struct Router {
    sender: ws::Sender,
    inner: Box<ws::Handler>,
}

impl Router {
    pub fn new(out: ws::Sender) -> Router {
        Router {
            sender: out,
            inner: Box::new(NotFound),
        }
    }
}

impl ws::Handler for Router {
    fn on_request(&mut self, req: &ws::Request) -> ws::Result<(ws::Response)> {
        // Clone the sender so that we can move it into the child handler
        let out = self.sender.clone();

        match req.resource() {
            "/events" => self.inner = Box::new(EventRouter { ws: out }),

            // Use the default child handler, NotFound
            _ => (),
        }

        // Delegate to the child handler
        self.inner.on_request(req)
    }

    fn on_shutdown(&mut self) {
        self.inner.on_shutdown()
    }

    fn on_open(&mut self, shake: ws::Handshake) -> ws::Result<()> {
        self.inner.on_open(shake)
    }

    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        self.inner.on_message(msg)
    }

    fn on_close(&mut self, code: ws::CloseCode, reason: &str) {
        self.inner.on_close(code, reason)
    }

    fn on_error(&mut self, err: ws::Error) {
        self.inner.on_error(err);
    }
}

// This handler returns a 404 response to all handshake requests
struct NotFound;

impl ws::Handler for NotFound {
    fn on_request(&mut self, req: &ws::Request) -> ws::Result<(ws::Response)> {
        // This handler responds to all requests with a 404
        let mut res = ws::Response::from_request(req)?;
        res.set_status(404);
        res.set_reason("Not Found");
        Ok(res)
    }
}

struct EventRouter {
    ws: ws::Sender,
}

impl ws::Handler for EventRouter {
    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        match msg.into_text() {
            Ok(s) => match parser::parse(s) {
                Ok(c) => self.ws.send(format!("Event received: {:?}", c)),
                Err(e) => self.ws.send(e),
            },
            Err(e) => Err(e),
        }
    }
}
