use anyhow::Result;
use ui9_dui::relay::MeshNode;
use ui9_dui::Hub;

pub struct Mesh {}

impl Mesh {
    pub async fn activate() -> Result<()> {
        Hub::activate().await?;
        MeshNode::activate().await?;
        Ok(())
    }

    pub async fn deactivate() -> Result<()> {
        MeshNode::deactivate().await?;
        Hub::deactivate().await?;
        Ok(())
    }
}
