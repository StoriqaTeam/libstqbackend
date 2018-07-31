use failure;
use futures::prelude::*;

pub type ApiFuture<T> = Box<Future<Item = T, Error = failure::Error> + Send>;

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ValueContainer<T> {
    pub value: T,
}

impl<T> From<T> for ValueContainer<T> {
    fn from(value: T) -> Self {
        Self { value }
    }
}
