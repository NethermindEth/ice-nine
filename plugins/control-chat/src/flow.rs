use ui9::names::Fqn;
use ui9_dui::{Flow, Listener, Subscriber, Tracer, Publisher, Unified};
use derive_more::{Deref, DerefMut, From, Into};
use serde::{Serialize, Deserialize};

#[derive(Deref, DerefMut, From, Into)]
pub struct ChatSub {
    listener: Listener<Chat>,
}

impl Subscriber for Chat {
    type Driver = ChatSub;
}

#[derive(Deref, DerefMut, From, Into)]
pub struct ChatPub {
    tracer: Tracer<Chat>,
}

impl Publisher for Chat {
    type Driver = ChatPub;
}

impl ChatPub {
    pub fn add(&mut self, message: String) {
        let event = ChatEvent::Add { message };
        self.tracer.event(event);
    }
}

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct Chat {
    messages: Vec<String>,
}

impl Unified for Chat {
    fn fqn() -> Fqn {
        Fqn::root("control-chat")
    }
}

impl Flow for Chat {
    type Event = ChatEvent;
    type Action = ();

    fn apply(&mut self, event: Self::Event) {
        match event {
            ChatEvent::Add { message } => {
                self.messages.push(message);
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum ChatEvent {
    Add { message: String },
}
