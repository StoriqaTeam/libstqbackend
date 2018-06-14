use connection::*;

use bb8;
use bb8_postgres;
use futures::prelude::*;
use tokio_postgres;

#[derive(Clone, Debug)]
pub struct Pool {
    inner: bb8::Pool<bb8_postgres::PostgresConnectionManager>,
}

impl Pool {
    pub fn run<F, U, T, E>(&self, f: F) -> impl Future<Item = T, Error = E>
    where
        F: FnOnce(BoxedConnection<E>) -> U + Send + 'static,
        U: IntoFuture<Item = (T, BoxedConnection<E>), Error = (E, BoxedConnection<E>)> + 'static,
        T: 'static,
        E: From<tokio_postgres::Error> + Send + Sync + 'static,
    {
        self.inner.run(move |conn| {
            f(Box::new(conn) as BoxedConnection<E>)
                .into_future()
                .map(|(v, conn)| (v, conn.unwrap_tokio_postgres()))
                .map_err(|(e, conn)| (e, conn.unwrap_tokio_postgres()))
        })
    }
}

impl From<bb8::Pool<bb8_postgres::PostgresConnectionManager>> for Pool {
    fn from(v: bb8::Pool<bb8_postgres::PostgresConnectionManager>) -> Self {
        Self { inner: v }
    }
}
