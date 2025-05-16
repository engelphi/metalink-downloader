use crate::Result;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{policies::ExponentialBackoff, Jitter, RetryTransientMiddleware};

use anyhow::Context;
use std::time::Duration;

pub(crate) type Client = ClientWithMiddleware;

/// Creates a reqwest client to be used by the downloader tasks
pub(crate) fn make_http_client(user_agent: String) -> Result<Client> {
    let retry_policy = ExponentialBackoff::builder()
        .retry_bounds(Duration::from_secs(1), Duration::from_secs(60))
        .jitter(Jitter::Bounded)
        .base(2)
        .build_with_max_retries(5);
    Ok(ClientBuilder::new(
        reqwest::ClientBuilder::new()
            .https_only(true)
            .http2_prior_knowledge()
            .gzip(true)
            .zstd(true)
            .timeout(Duration::from_secs(20))
            .user_agent(user_agent)
            .build()?,
    )
    .with(RetryTransientMiddleware::new_with_policy(retry_policy))
    .build())
}

pub(crate) async fn request_range(
    client: &Client,
    url: &reqwest::Url,
    start: u64,
    end: u64,
) -> Result<reqwest::Response> {
    use reqwest::header::{HeaderValue, ACCEPT, CONNECTION, RANGE};
    Ok(client
        .get(url.clone())
        .header(
            RANGE,
            HeaderValue::from_str(&format!("bytes={start}-{end}"))?,
        )
        .header(ACCEPT, HeaderValue::from_str("*/*")?)
        .header(CONNECTION, HeaderValue::from_str("keep-alive")?)
        .send()
        .await?)
}

pub(crate) async fn get_file_size(client: &Client, url: reqwest::Url) -> Result<Option<u64>> {
    let mut response = client.head(url).send().await?;

    match response
        .headers_mut()
        .entry(reqwest::header::CONTENT_LENGTH)
    {
        reqwest::header::Entry::Occupied(entry) => Ok(Some(
            entry
                .get()
                .to_str()
                .with_context(|| "Failed convert header Content-Length header value to string")?
                .parse()
                .with_context(|| "Failed to parse Content-Length header")?,
        )),
        reqwest::header::Entry::Vacant(_) => Ok(None),
    }
}
