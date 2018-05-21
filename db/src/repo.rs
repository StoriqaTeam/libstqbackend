use super::connection::*;
use super::statement::*;

use failure;
use futures::*;
use futures_state_stream::*;
use tokio_postgres::rows::Row;

pub trait Filter {
    fn into_filtered_operation_builder(self, op: FilteredOperation, table: &'static str) -> FilteredOperationBuilder;
}

pub trait Inserter {
    fn into_insert_builder(self, table: &'static str) -> InsertBuilder;
}

pub trait Updater {
    fn into_update_builder(self, table: &'static str) -> UpdateBuilder;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InsertError {
    NoData,
}

pub trait DbRepoInsert<T: Send + 'static, I: Inserter, E: From<InsertError> + Send + 'static>
     {
    fn insert(&self, conn: BoxedConnection<E>, inserter: I) -> ConnectionFuture<Vec<T>, E>;

    fn insert_exactly_one(&self, conn: BoxedConnection<E>, inserter: I) -> ConnectionFuture<T, E> {
        Box::new(self.insert(conn, inserter).and_then(|(mut data, conn)| match data.pop() {
            Some(v) => Ok((v, conn)),
            None => Err((E::from(InsertError::NoData), conn)),
        }))
    }
}

pub trait DbRepoSelect<T: Send, F: Filter, E: Send> {
    fn select(&self, conn: BoxedConnection<E>, filter: F) -> ConnectionFuture<Vec<T>, E>;
}

pub trait DbRepoUpdate<T: Send, U: Updater, E: Send> {
    fn update(&self, conn: BoxedConnection<E>, updater: U) -> ConnectionFuture<Vec<T>, E>;
}

pub trait DbRepoDelete<T: Send, F: Filter, E: Send> {
    fn delete(&self, conn: BoxedConnection<E>, filter: F) -> ConnectionFuture<Vec<T>, E>;
}

pub trait DbRepo<T: Send + 'static, I: Inserter, F: Filter, U: Updater, E: From<InsertError> + Send + 'static>
    : DbRepoInsert<T, I, E> + DbRepoSelect<T, F, E> + DbRepoUpdate<T, U, E> + DbRepoDelete<T, F, E>
    {
}

pub type RepoError = failure::Error;
pub type RepoFuture<T> = Box<Future<Item = T, Error = RepoError>>;
pub type RepoConnection = BoxedConnection<RepoError>;
pub type RepoConnectionFuture<T> = ConnectionFuture<T, RepoError>;

impl From<InsertError> for RepoError {
    fn from(v: InsertError) -> Self {
        match v {
            InsertError::NoData => format_err!("Insert operation returned no data"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct DbRepoImpl {
    pub table: &'static str,
}

impl DbRepoImpl {
    pub fn new(table: &'static str) -> Self {
        Self { table }
    }
}

impl<T, N> DbRepoInsert<T, N, RepoError> for DbRepoImpl
where
    T: From<Row> + Send + 'static,
    N: Inserter + Send,
{
    fn insert(&self, conn: RepoConnection, inserter: N) -> RepoConnectionFuture<Vec<T>> {
        let (statement, args) = inserter.into_insert_builder(self.table).build();

        Box::new(
            conn.prepare2(&statement)
                .and_then(move |(statement, conn)| conn.query2(&statement, args).collect())
                .map(move |(rows, conn)| (rows.into_iter().map(T::from).collect::<Vec<_>>(), conn)),
        )
    }
}

impl<T, F> DbRepoSelect<T, F, RepoError> for DbRepoImpl
where
    T: From<Row> + Send + 'static,
    F: Filter + Send,
{
    fn select(&self, conn: RepoConnection, mask: F) -> RepoConnectionFuture<Vec<T>> {
        let (statement, args) = mask.into_filtered_operation_builder(FilteredOperation::Select, self.table).build();

        Box::new(
            conn.prepare2(&statement)
                .and_then({ move |(statement, conn)| conn.query2(&statement, args).collect() })
                .map(|(rows, conn)| (rows.into_iter().map(T::from).collect::<_>(), conn)),
        )
    }
}

impl<T, U> DbRepoUpdate<T, U, RepoError> for DbRepoImpl
where
    T: From<Row> + Send + 'static,
    U: Updater + Send,
{
    fn update(&self, conn: RepoConnection, updater: U) -> RepoConnectionFuture<Vec<T>> {
        let (statement, args) = updater.into_update_builder(self.table).build();

        Box::new(
            conn.prepare2(&statement)
                .and_then(move |(statement, conn)| conn.query2(&statement, args).collect())
                .map(|(rows, conn)| (rows.into_iter().map(T::from).collect::<_>(), conn)),
        )
    }
}

impl<T, F> DbRepoDelete<T, F, RepoError> for DbRepoImpl
where
    T: From<Row> + Send + 'static,
    F: Filter + Send,
{
    fn delete(&self, conn: RepoConnection, filter: F) -> RepoConnectionFuture<Vec<T>> {
        let (statement, args) = filter
            .into_filtered_operation_builder(FilteredOperation::Delete, self.table)
            .build();

        Box::new(
            conn.prepare2(&statement)
                .and_then(move |(statement, conn)| conn.query2(&statement, args).collect())
                .map(|(rows, conn)| (rows.into_iter().map(T::from).collect::<_>(), conn)),
        )
    }
}

impl<T, N, F, U> DbRepo<T, N, F, U, RepoError> for DbRepoImpl
where
    T: From<Row> + Send + 'static,
    N: Inserter + Send,
    F: Filter + Send,
    U: Updater + Send,
{
}
