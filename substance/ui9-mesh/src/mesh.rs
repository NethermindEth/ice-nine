use anyhow::Result;
use ui9_dui::Hub;
use ui9_net::{MeshNode, RemoteGenerator};

pub struct Mesh {}

impl Mesh {
    pub async fn activate() -> Result<()> {
        Hub::activate(RemoteGenerator).await?;
        MeshNode::activate().await?;
        Ok(())
    }

    pub async fn deactivate() -> Result<()> {
        MeshNode::deactivate().await?;
        Hub::deactivate().await?;
        Ok(())
    }
}
