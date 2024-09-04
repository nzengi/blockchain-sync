use futures::prelude::*;
use libp2p::{
    development_transport, identity, mdns::{Mdns, MdnsConfig, MdnsEvent}, swarm::{SwarmBuilder, SwarmEvent}, 
    PeerId, Swarm, NetworkBehaviour,
};
use log::{info, error};
use std::error::Error;

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "P2PEvent")]
pub struct P2PBehaviour {
    pub mdns: Mdns,
    #[behaviour(ignore)]
    pub peer_id: PeerId,
}

impl P2PBehaviour {
    pub async fn new(peer_id: PeerId) -> Result<Self, Box<dyn Error>> {
        let mdns = Mdns::new(MdnsConfig::default()).await?;
        Ok(Self { mdns, peer_id })
    }
}

// Define the events we are interested in
#[derive(Debug)]
pub enum P2PEvent {
    Mdns(MdnsEvent),
}

impl From<MdnsEvent> for P2PEvent {
    fn from(event: MdnsEvent) -> Self {
        P2PEvent::Mdns(event)
    }
}

pub async fn start_p2p_node() -> Result<(), Box<dyn Error>> {
    // Create a random PeerId for the node
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());

    info!("Local peer id: {:?}", local_peer_id);

    // Create a transport (how nodes communicate)
    let transport = development_transport(local_key.clone()).await?;

    // Create swarm behaviour with mDNS
    let behaviour = P2PBehaviour::new(local_peer_id.clone()).await?;
    let mut swarm = SwarmBuilder::new(transport, behaviour, local_peer_id.clone()).build();

    info!("P2P node started. Listening for peers...");

    // Start listening for new peers and handle events
    loop {
        match swarm.next().await {
            Some(SwarmEvent::Behaviour(P2PEvent::Mdns(MdnsEvent::Discovered(peers)))) => {
                for (peer_id, _addr) in peers {
                    info!("Discovered peer: {:?}", peer_id);
                }
            },
            Some(SwarmEvent::Behaviour(P2PEvent::Mdns(MdnsEvent::Expired(peers)))) => {
                for (peer_id, _addr) in peers {
                    info!("Expired peer: {:?}", peer_id);
                }
            },
            Some(SwarmEvent::NewListenAddr { address, .. }) => {
                info!("Listening on {:?}", address);
            },
            _ => {}
        }
    }
}
