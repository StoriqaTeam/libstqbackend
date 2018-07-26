use failure;
use futures::{future, Future};
use serde::Serialize;
use serde_json;

pub fn serialize_payload<T>(v: T) -> impl Future<Item = String, Error = failure::Error>
where
    T: Serialize,
{
    future::result(serde_json::to_string(&v).map_err(failure::Error::from))
}
