mod downloader;

pub async fn start_sync() {
    // Use the download_blocks_with_limit function to start block download with a limit
    if let Err(e) = downloader::download_blocks_with_limit(1, 1000, 3).await {
        eprintln!("Error in block synchronization: {}", e);
    }
}
