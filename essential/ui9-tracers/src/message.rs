use serde::Serialize;
use ui9::flow::EventFlow;
use ui9::names::Fqn;
use ui9::tracer::Tracer;

#[derive(Serialize, Clone)]
pub struct MessageValue {
    content: String,
}

impl EventFlow for MessageValue {
    fn class() -> &'static str {
        "ui9.message"
    }
}

pub struct Message {
    tracer: Tracer<MessageValue>,
}

impl Message {
    pub fn new(fqn: Fqn) -> Self {
        let tracer = Tracer::new(fqn);
        Self { tracer }
    }

    pub fn set_value(&mut self, content: &str) {
        let message = MessageValue {
            content: content.to_owned(),
        };
    }
}
