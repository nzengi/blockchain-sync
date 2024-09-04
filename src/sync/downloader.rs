use futures::stream::{FuturesUnordered, StreamExt};
use tokio::task::JoinHandle;
use log::{info, error};


/// Example for the function that might need type annotations
pub async fn download_blocks_with_limit(start: u64, end: u64, limit: usize) -> Result<(), String> {
    // Bu satırdaki türü netleştiriyoruz
    let mut tasks: FuturesUnordered<JoinHandle<Result<(), String>>> = FuturesUnordered::new();

    for block_number in start..=end {
        if tasks.len() >= limit {
            // Wait for one task to finish before adding more
            if let Some(result) = tasks.next().await {
                result.map_err(|e| e.to_string())??;
            }
        }

        // Yeni bir blok indirme görevi ekliyoruz
        tasks.push(tokio::spawn(async move {
            download_single_block(block_number).await
        }));
    }

    // Kalan görevleri bekliyoruz
    while let Some(result) = tasks.next().await {
        result.map_err(|e| e.to_string())??;
    }

    Ok(())
}

async fn download_single_block(block_number: u64) -> Result<(), String> {
    // Örnek bir blok indirme işlemi
    if block_number % 100 == 0 {
        Err(format!("Failed to download block {}", block_number))
    } else {
        Ok(()) // Başarılı blok indirme işlemi
    }
}

