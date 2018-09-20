use util::*;

use hyper::{
    header::{HeaderName, HeaderValue},
    HeaderMap,
};
use reqwest::async::{Client as HttpClient, ClientBuilder as HttpClientBuilder};
use std::sync::Arc;
use stq_types::UserId;

#[derive(Clone, Debug)]
pub struct RestApiClient {
    pub(crate) http_client: Arc<HttpClient>,
    pub(crate) base_url: String,
}

impl RestApiClient {
    pub fn new<S>(base_url: &S, caller_id: Option<UserId>) -> Self
    where
        S: ToString,
    {
        Self {
            base_url: base_url.to_string(),
            http_client: Arc::new(
                HttpClientBuilder::new()
                    .default_headers(
                        match caller_id {
                            Some(v) => vec![(
                                HeaderName::from_static("authorization"),
                                HeaderValue::from_str(&v.to_string()).unwrap(),
                            )],
                            None => vec![],
                        }.into_iter()
                        .collect::<HeaderMap>(),
                    ).build()
                    .unwrap(),
            ),
        }
    }

    pub fn build_route(&self, route_builder: &RouteBuilder) -> String {
        route_builder.build_route(Some(&self.base_url))
    }
}
