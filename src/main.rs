mod sync;
mod validator;
mod network;

use log::info;
use env_logger;

#[tokio::main]
async fn main() {
    // Initialize the logger
    env_logger::init();

    info!("Starting blockchain synchronization with P2P...");

    // Start P2P network
    tokio::spawn(async {
        if let Err(e) = network::p2p::start_p2p_node().await {
            eprintln!("P2P node error: {}", e);
        }
    });

    // Start the sync process
    sync::start_sync().await;
}
