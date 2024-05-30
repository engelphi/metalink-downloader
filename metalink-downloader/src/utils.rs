use std::time::Duration;

use anyhow::Result;

pub fn make_http_client(user_agent: String) -> Result<reqwest::Client> {
    Ok(reqwest::ClientBuilder::new()
        .https_only(true)
        //.http2_prior_knowledge()
        .gzip(true)
        .timeout(Duration::from_secs(20))
        .user_agent(user_agent)
        .build()?)
}
