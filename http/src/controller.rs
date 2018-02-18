use failure::Fail;
use futures::prelude::*;
use futures::future;
use hyper;
use hyper::{Error, mime, StatusCode};
use hyper::header::{ContentLength, ContentType};
use hyper::server::{Request, Response, Service};
use serde_json;

use ErrorMessage;
use errors::ControllerError;
use request_util::ControllerFuture;

pub trait Controller {
    fn call(&self, request: Request) -> ControllerFuture;
}

pub type ServerFuture = Box<Future<Item = Response, Error = hyper::Error>>;

pub struct Application {
    pub controller: Box<Controller>,
}

impl Service for Application {
    type Request = Request;
    type Response = Response;
    type Error = Error;
    type Future = ServerFuture;

    fn call(&self, req: Request) -> ServerFuture {
        info!("{:?}", req);

        Box::new(self.controller.call(req).then(|res| match res {
            Ok(data) => future::ok(Self::response_with_json(data)),
            Err(err) => future::ok(Self::response_with_error(err)),
        }))
    }
}

impl Application {
    /// Responds with JSON, logs response body
    fn response_with_json(body: String) -> Response {
        info!("{}", body);

        Self::response_with_body(body)
    }

    /// Responds with JSON error, logs response body
    fn response_with_error(error: ControllerError) -> Response {
        if let Some(trace) = error.backtrace() {
            error!("Trace: {}", trace);
        }
        error!("{:?}", error);
        let mes = ErrorMessage {
            code: error.code().as_u16(),
            message: error.message(),
        };
        let mes = serde_json::to_string(&mes).unwrap();
        Self::response_with_body(mes).with_status(error.code())
    }

    fn response_with_body(body: String) -> Response {
        Response::new()
            .with_header(ContentLength(body.len() as u64))
            .with_header(ContentType(mime::APPLICATION_JSON))
            .with_status(StatusCode::Ok)
            .with_body(body)
    }
}
