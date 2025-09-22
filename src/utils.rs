use rand::Rng;
use tokio::time::{sleep, Duration};

/// Evade detection by sleeping for a random duration.
pub async fn evade() {
    let delay = rand::thread_rng().gen_range(2..=5);
    sleep(Duration::from_secs(delay)).await;
}

/// Extract ID from LinkedIn URN.
pub fn get_id_from_urn(urn: &str) -> &str {
    urn.split(':').nth(3).unwrap_or("")
}
