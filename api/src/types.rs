use failure;
use futures::prelude::*;

pub type ApiFuture<T> = Box<Future<Item = T, Error = failure::Error> + Send>;

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
