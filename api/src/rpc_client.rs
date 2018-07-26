use hyper::{header::{HeaderName, HeaderValue},
            HeaderMap};
use reqwest::unstable::async::{Client as HttpClient, ClientBuilder as HttpClientBuilder};
use std::sync::Arc;
use stq_types::UserId;

#[derive(Clone, Debug)]
pub struct RpcClientImpl {
    pub(crate) http_client: Arc<HttpClient>,
    pub(crate) base_url: String,
}

impl RpcClientImpl {
    pub fn new<S>(base_url: S, caller_id: UserId) -> Self
    where
        S: ToString,
    {
        Self {
            base_url: base_url.to_string(),
            http_client: Arc::new(
                HttpClientBuilder::new()
                    .default_headers(
                        vec![(
                            HeaderName::from_static("Authorization"),
                            HeaderValue::from_str(&caller_id.to_string()).unwrap(),
                        )].into_iter()
                            .collect::<HeaderMap>(),
                    )
                    .build()
                    .unwrap(),
            ),
        }
    }
}
