use futures::future;

use request_util::ControllerFuture;

/// System service, responsible for common endpoints like healthcheck
pub trait SystemService {
    /// Healthcheck endpoint, always returns OK status
    fn healthcheck(&self) -> ControllerFuture;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SystemServiceImpl;

impl SystemService for SystemServiceImpl {
    /// Healthcheck endpoint, always returns OK status
    fn healthcheck(&self) -> ControllerFuture {
        Box::new(future::ok("\"Ok\"".to_string()))
    }
}
