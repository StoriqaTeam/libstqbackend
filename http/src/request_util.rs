use failure;
use failure::Fail;
use futures::future;
use futures::prelude::*;
use hyper;
use serde::de::Deserialize;
use serde::ser::Serialize;
use serde_json;

#[derive(Clone, Debug, Fail)]
pub enum ParseError {
    #[fail(display = "Failure while reading body")]
    ReadError,
    #[fail(display = "Failed to convert received body")]
    ConvertError,
}

/// Transforms request body with the following pipeline:
///
///   1. Parse request body into entity of type T (T must implement `serde::de::Deserialize` trait)
///
///   2. Validate entity (T must implement `validator::Validate`)
///
/// Fails with `error::Error::UnprocessableEntity` if step 1 fails.
///
/// Fails with `error::Error::BadRequest` with message if step 2 fails.
pub fn parse_body<T>(body: hyper::Body) -> Box<Future<Item = T, Error = failure::Error>>
where
    T: for<'a> Deserialize<'a> + 'static,
{
    Box::new(
        read_body(body)
            .map_err(|err| err.context(ParseError::ReadError).into())
            .and_then(move |body| {
                serde_json::from_str::<T>(&body).map_err(move |err| {
                    err.context(format!("Failed to parse as JSON: {}", body))
                        .context(ParseError::ConvertError)
                        .into()
                })
            }),
    )
}

/// Reads body of request and response in Future format
pub fn read_body(body: hyper::Body) -> Box<Future<Item = String, Error = hyper::Error>> {
    Box::new(body.fold(Vec::new(), |mut acc, chunk| {
        acc.extend_from_slice(&*chunk);
        future::ok::<_, hyper::Error>(acc)
    }).and_then(|bytes| match String::from_utf8(bytes) {
        Ok(data) => future::ok(data),
        Err(err) => future::err(hyper::Error::Utf8(err.utf8_error())),
    }))
}

pub fn serialize_future<T, E, F>(f: F) -> Box<Future<Item = String, Error = failure::Error>>
where
    F: IntoFuture<Item = T, Error = E> + 'static,
    E: 'static,
    failure::Error: From<E>,
    T: Serialize,
{
    Box::new(
        f.into_future()
            .map_err(failure::Error::from)
            .and_then(|resp| serde_json::to_string(&resp).map_err(|e| e.into())),
    )
}
