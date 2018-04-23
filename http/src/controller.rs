use failure::Fail;
use futures::future;
use futures::prelude::*;
use hyper;
use hyper::header::{AccessControlAllowHeaders, AccessControlAllowMethods, AccessControlAllowOrigin, AccessControlRequestHeaders,
                    ContentLength, ContentType};
use hyper::server::{Request, Response, Service};
use hyper::Method::{Get, Options, Post};
use hyper::{mime, Error, Headers, StatusCode};
use serde_json;
use std;

use errors::{ControllerError, ErrorMessage};
use request_util::ControllerFuture;

pub trait Controller {
    fn call(&self, request: Request) -> ControllerFuture;
}

pub type ServerFuture = Box<Future<Item = Response, Error = hyper::Error>>;

pub struct Application {
    pub controller: Box<Controller>,
    pub acao: AccessControlAllowOrigin,
}

impl Service for Application {
    type Request = Request;
    type Response = Response;
    type Error = Error;
    type Future = ServerFuture;

    fn call(&self, req: Request) -> ServerFuture {
        debug!("Received request: {:?}", req);

        match req.method() {
            &Options => {
                let req_headers = req.headers().clone();
                let acah = req_headers.get::<AccessControlRequestHeaders>();

                let mut resp = Response::new();
                let mut new_headers = Headers::new();
                new_headers.set(self.acao.clone());
                new_headers.set(AccessControlAllowMethods(vec![Get, Post, Options]));
                if let Some(a) = acah {
                    new_headers.set(AccessControlAllowHeaders(a.to_vec()));
                };
                new_headers.set(ContentType(mime::TEXT_HTML));

                std::mem::replace(resp.headers_mut(), new_headers);

                Box::new(future::ok(resp))
            }
            _ => Box::new(
                self.controller
                    .call(req)
                    .then({
                        let acao = self.acao.clone();
                        |res| match res {
                            Ok(data) => future::ok(Self::response_with_json(data, acao)),
                            Err(err) => future::ok(Self::response_with_error(err, acao)),
                        }
                    })
                    .inspect(|resp| debug!("Sending response: {:?}", resp)),
            ),
        }
    }
}

impl Application {
    pub fn new<T>(controller: T) -> Self
    where
        T: Controller + 'static,
    {
        Self {
            controller: Box::new(controller),
            acao: AccessControlAllowOrigin::Any,
        }
    }

    pub fn with_controller<T>(mut self, controller: T) -> Self
    where
        T: Controller + 'static,
    {
        self.controller = Box::new(controller);
        self
    }

    pub fn with_acao(mut self, acao: AccessControlAllowOrigin) -> Self {
        self.acao = acao;
        self
    }

    /// Responds with JSON, logs response body
    fn response_with_json(body: String, acao: AccessControlAllowOrigin) -> Response {
        info!("{}", body);

        Self::response_with_body(body, acao)
    }

    /// Responds with JSON error, logs response body
    fn response_with_error(error: ControllerError, acao: AccessControlAllowOrigin) -> Response {
        if let Some(trace) = error.backtrace() {
            error!("Trace: {}", trace);
        }
        error!("{}", error);
        let mes = ErrorMessage {
            code: error.code().as_u16(),
            message: error.to_string(),
        };
        let mes = serde_json::to_string(&mes).unwrap();
        Self::response_with_body(mes, acao).with_status(error.code())
    }

    fn response_with_body(body: String, acao: AccessControlAllowOrigin) -> Response {
        Response::new()
            .with_header(ContentLength(body.len() as u64))
            .with_header(ContentType(mime::APPLICATION_JSON))
            .with_header(acao)
            .with_status(StatusCode::Ok)
            .with_body(body)
    }
}
