use failure;
use futures::{future, Future};
use reqwest::async::RequestBuilder;
use serde::{de::DeserializeOwned, Serialize};
use serde_json;

pub fn serialize_payload<T>(v: T) -> impl Future<Item = String, Error = failure::Error>
where
    T: Serialize,
{
    future::result(serde_json::to_string(&v).map_err(failure::Error::from))
}

pub fn http_req<T>(b: RequestBuilder) -> Box<Future<Item = T, Error = failure::Error> + Send>
where
    T: DeserializeOwned + Send + 'static,
{
    Box::new(
        b.send()
            .map_err(failure::Error::from)
            .and_then(|mut rsp| rsp.json().map_err(failure::Error::from)),
    )
}

pub trait RouteBuilder {
    fn route(&self) -> String;

    fn build_route(&self, base: Option<&AsRef<str>>) -> String {
        {
            format!(
                "{}{}",
                match base {
                    Some(url) => format!("{}/", url.as_ref()),
                    None => "".to_string(),
                },
                self.route()
            )
        }
    }
}
