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

pub struct MessageTracer {
    tracer: Tracer<MessageValue>,
}

impl MessageTracer {
    pub fn new(fqn: Fqn) -> Self {
        let tracer = Tracer::new(fqn);
        Self { tracer }
    }

    pub fn add_message(&mut self, content: &str) {
        let message = MessageValue {
            content: content.to_owned(),
        };
        self.tracer.trace(&message);
    }
}
