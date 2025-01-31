use anyhow::Result;
use tracing_subscriber::EnvFilter;
use libp2p::identity::PeerId;
use libp2p::swarm::Swarm;
use libp2p_swarm_test::SwarmExt;

#[tokio::test]
async fn ping_flow() -> Result<()> {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init();

    let offline_peer = PeerId::random();

    /*
    let mut swarm1 = Swarm::new_ephemeral(|_| {
    });
    */

    Ok(())
}
