use futures_util::StreamExt;
use reth::network::{NetworkEvent, NetworkEvents, NetworkHandle, PeersInfo};
use reth_chainspec::net::NodeRecord;
use reth_tokio_util::EventStream;
use reth_tracing::tracing::info;

/// Helper for network operations
pub struct NetworkTestContext {
    network_events: EventStream<NetworkEvent>,
    network: NetworkHandle,
}

impl NetworkTestContext {
    /// Creates a new network helper
    pub fn new(network: NetworkHandle) -> Self {
        let network_events = network.event_listener();
        Self { network_events, network }
    }

    /// Adds a peer to the network node via network handle
    pub async fn add_peer(&mut self, node_record: NodeRecord) {
        self.network.peers_handle().add_peer(node_record.id, node_record.tcp_addr());

        match self.network_events.next().await {
            Some(NetworkEvent::PeerAdded(_)) => (),
            ev => panic!("Expected a peer added event, got: {ev:?}"),
        }
    }

    /// Returns the network node record
    pub fn record(&self) -> NodeRecord {
        self.network.local_node_record()
    }

    /// Expects a session to be established
    pub async fn expect_session(&mut self) {
        match self.network_events.next().await {
            Some(NetworkEvent::SessionEstablished { remote_addr, .. }) => {
                info!(?remote_addr, "Session established")
            }
            ev => panic!("Expected session established event, got: {ev:?}"),
        }
    }
}
