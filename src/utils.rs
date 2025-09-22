use rand::Rng;
use tokio::time::{sleep, Duration};

use crate::UniformResourceName;

/// Evade detection by sleeping for a random duration.
pub async fn evade() {
    let delay = rand::thread_rng().gen_range(2..=5);
    sleep(Duration::from_secs(delay)).await;
}

/// Extract ID
pub(crate) fn get_id_from_urn(urn: Option<UniformResourceName>) -> Option<String> {
    if let Some(secure_urn) = urn {
        return Some(secure_urn.id);
    } else {
        return None;
    }
}
