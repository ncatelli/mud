extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EventType {
    Game,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    event: EventType,
    message: String,
}

impl Event {
    pub fn new(event: EventType, message: String) -> Event {
        Event {
            event: event,
            message: message,
        }
    }

    pub fn event(&self) -> EventType {
        self.event.clone()
    }

    pub fn message(&self) -> String {
        self.message.clone()
    }
}
