use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession};
use crb::superagent::{OnRequest, Request};
use ice_nine_core::{SubstanceLinks, Tool, ToolResponse};
use serde::Deserialize;

pub struct DyDxToolPrice {
    substance: SubstanceLinks,
}

impl Agent for DyDxToolPrice {
    type Context = AgentSession<Self>;
    type Output = ();
}

impl Tool for DyDxToolPrice {
    type Request = Price;
}

#[derive(Deserialize)]
pub struct Price {
    ticker: String,
}

impl Request for Price {
    type Response = ToolResponse;
}

#[async_trait]
impl OnRequest<Price> for DyDxToolPrice {
    async fn on_request(&mut self, msg: Price, _: &mut Self::Context) -> Result<ToolResponse> {
        todo!()
    }
}
