use super::connection::*;
use super::statement::*;

use failure;
use futures::*;
use futures_state_stream::*;
use std::marker::PhantomData;
use tokio_postgres;

pub trait Filter {
    fn into_filtered_operation_builder(self, op: FilteredOperation, table: &'static str) -> FilteredOperationBuilder;
}

pub trait Inserter {
    fn into_insert_builder(self, table: &'static str) -> InsertBuilder;
}

pub trait Updater {
    fn into_update_builder(self, table: &'static str) -> UpdateBuilder;
}

pub trait DbRepo<T, N: Inserter, F: Filter, U: Updater, E> {
    fn create(&self, conn: BoxedConnection<E>, inserter: N) -> ConnectionFuture<T, E>;
    fn get(&self, conn: BoxedConnection<E>, filter: F) -> ConnectionFuture<Vec<T>, E>;
    fn update(&self, conn: BoxedConnection<E>, updater: U) -> ConnectionFuture<Vec<T>, E>;
    fn remove(&self, conn: BoxedConnection<E>, filter: F) -> ConnectionFuture<Vec<T>, E>;
}

pub type RepoError = failure::Error;
pub type RepoFuture<T> = Box<Future<Item = T, Error = RepoError>>;
pub type RepoConnection = BoxedConnection<RepoError>;
pub type RepoConnectionFuture<T> = ConnectionFuture<T, RepoError>;

#[derive(Clone, Debug)]
pub struct DbRepoImpl<T, N, F, U> {
    pub table: &'static str,
    t_type: PhantomData<T>,
    n_type: PhantomData<N>,
    f_type: PhantomData<F>,
    u_type: PhantomData<U>,
}

impl<T, N, F, U> DbRepoImpl<T, N, F, U> {
    pub fn new(table: &'static str) -> Self {
        Self {
            table,
            t_type: Default::default(),
            n_type: Default::default(),
            f_type: Default::default(),
            u_type: Default::default(),
        }
    }
}

impl<T, N, F, U> DbRepo<T, N, F, U, RepoError> for DbRepoImpl<T, N, F, U>
where
    T: From<tokio_postgres::rows::Row> + Send + 'static,
    N: Inserter + Send,
    F: Filter + Send,
    U: Updater + Send,
{
    fn create(&self, conn: RepoConnection, new_item: N) -> RepoConnectionFuture<T> {
        let (statement, args) = new_item.into_insert_builder(self.table).build();

        Box::new(
            conn.prepare2(&statement)
                .and_then(move |(statement, conn)| conn.query2(&statement, args).collect())
                .and_then({
                    let statement = statement.clone();
                    move |(mut rows, conn)| match rows.pop() {
                        Some(row) => Ok((T::from(row), conn)),
                        None => Err((
                            format_err!("Insert op returned no rows: statement: {}", &statement),
                            conn,
                        )),
                    }
                }),
        )
    }

    fn get(&self, conn: RepoConnection, mask: F) -> RepoConnectionFuture<Vec<T>> {
        let (statement, args) = mask.into_filtered_operation_builder(FilteredOperation::Select, self.table)
            .build();

        Box::new(
            conn.prepare2(&statement)
                .and_then({ move |(statement, conn)| conn.query2(&statement, args).collect() })
                .map(|(rows, conn)| (rows.into_iter().map(T::from).collect::<_>(), conn)),
        )
    }

    fn update(&self, conn: RepoConnection, updater: U) -> RepoConnectionFuture<Vec<T>> {
        let (statement, args) = updater.into_update_builder(self.table).build();

        Box::new(
            conn.prepare2(&statement)
                .and_then(move |(statement, conn)| conn.query2(&statement, args).collect())
                .map(|(rows, conn)| (rows.into_iter().map(T::from).collect::<_>(), conn)),
        )
    }

    fn remove(&self, conn: RepoConnection, mask: F) -> RepoConnectionFuture<Vec<T>> {
        let (statement, args) = mask.into_filtered_operation_builder(FilteredOperation::Delete, self.table)
            .build();

        Box::new(
            conn.prepare2(&statement)
                .and_then(move |(statement, conn)| conn.query2(&statement, args).collect())
                .map(|(rows, conn)| (rows.into_iter().map(T::from).collect::<_>(), conn)),
        )
    }
}
