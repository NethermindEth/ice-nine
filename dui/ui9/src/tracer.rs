use crate::flow::EventFlow;
use crate::names::{FlowId, Fqn, GENERATOR};
use crate::protocol::{Event, Record};
use std::marker::PhantomData;

pub struct Tracer<OUT: EventFlow> {
    id: FlowId,
    fqn: Fqn,
    _out: PhantomData<OUT>,
}

impl<OUT> Tracer<OUT>
where
    OUT: EventFlow,
{
    pub fn new(fqn: Fqn) -> Self {
        let this = Self {
            id: GENERATOR.next(),
            fqn,
            _out: PhantomData,
        };
        this.create();
        this
    }

    fn create(&self) {
        let class = OUT::class();
        let event = Event::Create {
            fqn: &self.fqn,
            class,
        };
        self.event(event);
    }

    pub fn trace(&self, value: &OUT) {
        let event = Event::Value(value);
        self.event(event);
    }

    fn destroy(&self) {
        let event = Event::Destroy;
        self.event(event);
    }

    fn event(&self, event: Event<OUT>) {
        let record = Record { id: self.id, event };
        if let Ok(dump) = serde_json::to_string(&record) {
            println!("{}", dump);
        }
    }
}

impl<OUT> Drop for Tracer<OUT>
where
    OUT: EventFlow,
{
    fn drop(&mut self) {
        self.destroy();
    }
}
