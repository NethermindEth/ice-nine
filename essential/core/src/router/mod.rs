pub mod model;

use anyhow::{Error, Result};
use async_trait::async_trait;
use crb::agent::{Address, Agent, AgentSession, Next, OnEvent, OnRequest, Request};
use derive_more::{Deref, DerefMut, From};
use model::{Model, ModelClient};
use std::collections::HashMap;

#[derive(Deref, DerefMut, From, Clone)]
pub struct RouterLink {
    address: Address<Router>,
}

pub struct Router {
    models: HashMap<String, ModelClient>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            models: HashMap::new(),
        }
    }
}

impl Agent for Router {
    type Context = AgentSession<Self>;
    type Output = ();

    fn begin(&mut self) -> Next<Self> {
        Next::events()
    }
}

pub struct AddModel {}

#[async_trait]
impl OnEvent<AddModel> for Router {
    type Error = Error;

    async fn handle(&mut self, _: AddModel, _ctx: &mut Self::Context) -> Result<()> {
        Ok(())
    }
}

pub struct TextRequest {
    text: String,
}

impl Request for TextRequest {
    type Response = ();
}

#[async_trait]
impl OnRequest<TextRequest> for Router {
    async fn on_request(&mut self, _lookup: TextRequest, _: &mut Self::Context) -> Result<()> {
        Ok(())
    }
}
