use super::connection::*;
use super::statement::*;

use failure;
use futures::*;
use futures_state_stream::*;
use std::sync::Arc;
use stq_acl as acl;
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

#[derive(Clone, Debug, PartialEq, Fail)]
pub enum SelectError {
    #[fail(display = "Denied select")]
    Unauthorized,
}

#[derive(Clone, Debug, PartialEq, Fail)]
pub enum UpdateError {
    #[fail(display = "Denied update")]
    Unauthorized,
}

#[derive(Clone, Debug, PartialEq, Fail)]
pub enum InsertError {
    #[fail(display = "Denied insert")]
    Unauthorized,
    #[fail(display = "Insert operation has returned no data.")]
    NoData,
    #[fail(display = "Insert operation returned extra data: +{}", extra)]
    ExtraData { extra: u32 },
}

#[derive(Clone, Debug, PartialEq, Fail)]
pub enum DeleteError {
    #[fail(display = "Denied delete")]
    Unauthorized,
    #[fail(display = "Delete operation has returned no data")]
    NoData,
    #[fail(display = "Delete operation returned extra data: +{}", extra)]
    ExtraData { extra: u32 },
}

pub trait DbRepoInsert<T: Send + 'static, I: Inserter, E: From<InsertError> + Send + 'static> {
    fn insert(&self, conn: BoxedConnection<E>, inserter: I) -> ConnectionFuture<Vec<T>, E>;

    fn insert_exactly_one(&self, conn: BoxedConnection<E>, inserter: I) -> ConnectionFuture<T, E> {
        Box::new(self.insert(conn, inserter).and_then(|(mut data, conn)| {
            if data.len() > 1 {
                return Err((
                    E::from(InsertError::ExtraData {
                        extra: data.len() as u32 - 1,
                    }),
                    conn,
                ));
            } else if data.len() == 0 {
                return Err((E::from(InsertError::NoData), conn));
            } else if data.len() == 1 {
                return Ok((data.pop().unwrap(), conn));
            } else {
                unreachable!()
            }
        }))
    }
}

pub trait DbRepoSelect<T: Send + 'static, F: Filter, E: From<SelectError> + Send> {
    fn select(&self, conn: BoxedConnection<E>, filter: F) -> ConnectionFuture<Vec<T>, E>;
}

pub trait DbRepoUpdate<T: Send + 'static, U: Updater, E: From<UpdateError> + Send> {
    fn update(&self, conn: BoxedConnection<E>, updater: U) -> ConnectionFuture<Vec<T>, E>;
}

pub trait DbRepoDelete<T: Send + 'static, F: Filter, E: From<DeleteError> + Send + 'static> {
    fn delete(&self, conn: BoxedConnection<E>, filter: F) -> ConnectionFuture<Vec<T>, E>;

    fn delete_exactly_one(&self, conn: BoxedConnection<E>, filter: F) -> ConnectionFuture<T, E> {
        Box::new(self.delete(conn, filter).and_then(|(mut data, conn)| {
            if data.len() > 1 {
                return Err((
                    E::from(DeleteError::ExtraData {
                        extra: data.len() as u32 - 1,
                    }),
                    conn,
                ));
            } else if data.len() == 0 {
                return Err((E::from(DeleteError::NoData), conn));
            } else if data.len() == 1 {
                return Ok((data.pop().unwrap(), conn));
            } else {
                unreachable!()
            }
        }))
    }
}

pub trait ImmutableDbRepo<
    T: Send + 'static,
    I: Inserter,
    F: Filter,
    E: From<InsertError> + From<SelectError> + From<DeleteError> + Send + 'static,
>: DbRepoInsert<T, I, E> + DbRepoSelect<T, F, E> + DbRepoDelete<T, F, E>
{
}

pub trait DbRepo<
    T: Send + 'static,
    I: Inserter,
    F: Filter,
    U: Updater,
    E: From<InsertError> + From<SelectError> + From<UpdateError> + From<DeleteError> + Send + 'static,
>: ImmutableDbRepo<T, I, F, E> + DbRepoUpdate<T, U, E>
{
}

pub type RepoError = failure::Error;
pub type RepoFuture<T> = Box<Future<Item = T, Error = RepoError>>;
pub type RepoConnection = BoxedConnection<RepoError>;
pub type RepoConnectionFuture<T> = ConnectionFuture<T, RepoError>;

#[derive(Clone, Copy, Debug, PartialEq, Hash)]
pub enum Action {
    Select,
    Insert,
    Update,
    Delete,
}

#[derive(Clone, Debug)]
pub struct DbRepoImpl {
    pub table: &'static str,
    pub acl_engine: Arc<acl::AclEngine<Action, RepoError>>,
}

impl DbRepoImpl {
    pub fn new(table: &'static str) -> Self {
        Self {
            table,
            acl_engine: Arc::new(acl::SystemACL::default()),
        }
    }

    pub fn with_acl_engine<E>(mut self, acl_engine: E) -> Self
    where
        E: acl::AclEngine<Action, RepoError> + 'static,
    {
        self.acl_engine = Arc::new(acl_engine);
        self
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
            self.acl_engine
                .ensure_access(Action::Insert)
                .then(move |res| {
                    future::result(match res {
                        Ok(ctx) => Ok((ctx, conn)),
                        Err(e) => Err((e, conn)),
                    })
                })
                .and_then(move |(_ctx, conn)| {
                    conn.prepare2(&statement)
                        .and_then(move |(statement, conn)| conn.query2(&statement, args).collect())
                        .map(move |(rows, conn)| (rows.into_iter().map(T::from).collect::<Vec<_>>(), conn))
                })
                .map_err(|(e, conn)| (e.context("Failure while running insert").into(), conn)),
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
            self.acl_engine
                .ensure_access(Action::Select)
                .then(move |res| {
                    future::result(match res {
                        Ok(ctx) => Ok((ctx, conn)),
                        Err(e) => Err((e, conn)),
                    })
                })
                .and_then(move |(_ctx, conn)| {
                    conn.prepare2(&statement)
                        .and_then({ move |(statement, conn)| conn.query2(&statement, args).collect() })
                        .map(|(rows, conn)| (rows.into_iter().map(T::from).collect::<_>(), conn))
                }),
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
            self.acl_engine
                .ensure_access(Action::Update)
                .then(move |res| {
                    future::result(match res {
                        Ok(ctx) => Ok((ctx, conn)),
                        Err(e) => Err((e, conn)),
                    })
                })
                .and_then(move |(_ctx, conn)| {
                    conn.prepare2(&statement)
                        .and_then(move |(statement, conn)| conn.query2(&statement, args).collect())
                        .map(|(rows, conn)| (rows.into_iter().map(T::from).collect::<_>(), conn))
                })
                .map_err(|(e, conn)| (e.context("Failure while running update").into(), conn)),
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
            self.acl_engine
                .ensure_access(Action::Delete)
                .then(move |res| {
                    future::result(match res {
                        Ok(ctx) => Ok((ctx, conn)),
                        Err(e) => Err((e, conn)),
                    })
                })
                .and_then(move |(_ctx, conn)| {
                    conn.prepare2(&statement)
                        .and_then(move |(statement, conn)| conn.query2(&statement, args).collect())
                        .map(|(rows, conn)| (rows.into_iter().map(T::from).collect::<_>(), conn))
                }),
        )
    }
}

impl<T, N, F> ImmutableDbRepo<T, N, F, RepoError> for DbRepoImpl
where
    T: From<Row> + Send + 'static,
    N: Inserter + Send,
    F: Filter + Send,
{
}

impl<T, N, F, U> DbRepo<T, N, F, U, RepoError> for DbRepoImpl
where
    T: From<Row> + Send + 'static,
    N: Inserter + Send,
    F: Filter + Send,
    U: Updater + Send,
{
}
