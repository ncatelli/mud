extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    event: String,
    message: String,
}

pub enum EventType {
    Message,
}

impl Event {
    pub fn new(event: String, message: String) -> Event {
        Event {
            event: event,
            message: message,
        }
    }

    pub fn event(&self) -> Result<EventType, &'static str> {
        match &*self.event {
            "message" => Ok(EventType::Message),
            _ => Err("Invalid event type"),
        }
    }

    pub fn message(&self) -> String {
        self.message.clone()
    }
}
