use derive_more::{Deref, DerefMut, From, Into};
use serde::{Deserialize, Serialize};
use ui9::names::Fqn;
use ui9_dui::{Flow, Listener, Publisher, Subscriber, Tracer, Unified};

#[derive(Deref, DerefMut, From, Into)]
pub struct ChatSub {
    listener: Listener<Chat>,
}

impl Subscriber for Chat {
    type Driver = ChatSub;
}

impl ChatSub {
    pub fn request(&mut self, question: String) {
        let event = ChatAction::Request { question };
        self.listener.action(event);
    }
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

    pub fn start_thinking(&mut self, reason: &str) {
        let reason = Some(reason.into());
        let event = ChatEvent::SetThinking { reason };
        self.tracer.event(event);
    }

    pub fn stop_thinking(&mut self) {
        let event = ChatEvent::SetThinking { reason: None };
        self.tracer.event(event);
    }
}

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct Chat {
    pub thinking: Option<String>,
    pub messages: Vec<String>,
}

impl Unified for Chat {
    fn fqn() -> Fqn {
        Fqn::root("control-chat")
    }
}

impl Flow for Chat {
    type Event = ChatEvent;
    type Action = ChatAction;

    fn apply(&mut self, event: Self::Event) {
        match event {
            ChatEvent::Add { message } => {
                self.messages.push(message);
            }
            ChatEvent::SetThinking { reason } => {
                self.thinking = reason;
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum ChatEvent {
    Add { message: String },
    SetThinking { reason: Option<String> },
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum ChatAction {
    Request { question: String },
}
