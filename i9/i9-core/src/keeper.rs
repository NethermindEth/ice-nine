use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, OnRequest, Request};

pub struct Keeper {}

impl Agent for Keeper {
    type Context = AgentSession<Self>;
    type Output = ();
}

pub struct GetSecret {}

impl Request for GetSecret {
    type Response = String;
}

impl OnRequest<GetSecret> for Keeper {}
