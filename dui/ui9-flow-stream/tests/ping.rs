use anyhow::Result;
use tracing_subscriber::EnvFilter;
use libp2p::identity::PeerId;
use libp2p::swarm::Swarm;
use libp2p_swarm_test::SwarmExt;
use ui9_flow_stream::{Behaviour, Cbor};

#[tokio::test]
async fn ping_flow() -> Result<()> {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init();
    log::trace!("Ping Test");

    let offline_peer = PeerId::random();

    let mut swarm1 = Swarm::new_ephemeral(|_| {
        Behaviour::<Cbor<(), ()>>::new()
    });

    let request_id1 = swarm1
        .behaviour_mut()
        .send_request(&offline_peer, ());

    Ok(())
}
