use super::connection::*;
use super::statement::{Filter, FilteredOperation, Inserter, Updater};

use failure;
use futures::*;
use futures_state_stream::*;
use std::rc::Rc;
use stq_acl as acl;
use tokio_postgres::rows::Row;
use tokio_postgres::types::ToSql;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Fail)]
pub enum MultipleOperationError {
    #[fail(display = "Operation has returned no data")]
    NoData,
    #[fail(display = "Operation returned extra data: +{}", extra)]
    ExtraData { extra: u32 },
}

pub trait DbRepoInsert<T: 'static, I: Inserter, E: From<MultipleOperationError> + 'static> {
    fn insert(&self, conn: BoxedConnection<E>, inserter: I) -> ConnectionFuture<Vec<T>, E>;

    fn insert_exactly_one(&self, conn: BoxedConnection<E>, inserter: I) -> ConnectionFuture<T, E> {
        Box::new(self.insert(conn, inserter).and_then(|(mut data, conn)| {
            if data.len() > 1 {
                return Err((
                    E::from(MultipleOperationError::ExtraData {
                        extra: data.len() as u32 - 1,
                    }),
                    conn,
                ));
            } else if data.len() == 0 {
                return Err((E::from(MultipleOperationError::NoData), conn));
            } else if data.len() == 1 {
                return Ok((data.pop().unwrap(), conn));
            } else {
                unreachable!()
            }
        }))
    }
}

pub trait DbRepoSelect<T: 'static, F: Filter, E: From<MultipleOperationError> + 'static> {
    fn select(&self, conn: BoxedConnection<E>, filter: F) -> ConnectionFuture<Vec<T>, E>;

    fn select_exactly_one(&self, conn: BoxedConnection<E>, filter: F) -> ConnectionFuture<T, E> {
        Box::new(self.select(conn, filter).and_then(|(mut data, conn)| {
            if data.len() > 1 {
                return Err((
                    E::from(MultipleOperationError::ExtraData {
                        extra: data.len() as u32 - 1,
                    }),
                    conn,
                ));
            } else if data.len() == 0 {
                return Err((E::from(MultipleOperationError::NoData), conn));
            } else if data.len() == 1 {
                return Ok((data.pop().unwrap(), conn));
            } else {
                unreachable!()
            }
        }))
    }
}

pub trait DbRepoUpdate<T: 'static, U: Updater, E: From<MultipleOperationError> + 'static> {
    fn update(&self, conn: BoxedConnection<E>, updater: U) -> ConnectionFuture<Vec<T>, E>;

    fn update_exactly_one(&self, conn: BoxedConnection<E>, updater: U) -> ConnectionFuture<T, E> {
        Box::new(self.update(conn, updater).and_then(|(mut data, conn)| {
            if data.len() > 1 {
                return Err((
                    E::from(MultipleOperationError::ExtraData {
                        extra: data.len() as u32 - 1,
                    }),
                    conn,
                ));
            } else if data.len() == 0 {
                return Err((E::from(MultipleOperationError::NoData), conn));
            } else if data.len() == 1 {
                return Ok((data.pop().unwrap(), conn));
            } else {
                unreachable!()
            }
        }))
    }
}

pub trait DbRepoDelete<T: 'static, F: Filter, E: From<MultipleOperationError> + 'static> {
    fn delete(&self, conn: BoxedConnection<E>, filter: F) -> ConnectionFuture<Vec<T>, E>;

    fn delete_exactly_one(&self, conn: BoxedConnection<E>, filter: F) -> ConnectionFuture<T, E> {
        Box::new(self.delete(conn, filter).and_then(|(mut data, conn)| {
            if data.len() > 1 {
                return Err((
                    E::from(MultipleOperationError::ExtraData {
                        extra: data.len() as u32 - 1,
                    }),
                    conn,
                ));
            } else if data.len() == 0 {
                return Err((E::from(MultipleOperationError::NoData), conn));
            } else if data.len() == 1 {
                return Ok((data.pop().unwrap(), conn));
            } else {
                unreachable!()
            }
        }))
    }
}

pub trait DbRepo<T: 'static, I: Inserter, F: Filter, U: Updater, E: From<MultipleOperationError> + 'static>:
    DbRepoInsert<T, I, E> + DbRepoSelect<T, F, E> + DbRepoDelete<T, F, E> + DbRepoUpdate<T, U, E>
{
}

pub type RepoError = failure::Error;
pub type RepoFuture<T> = Box<Future<Item = T, Error = RepoError>>;
pub type RepoConnection = BoxedConnection<RepoError>;
pub type RepoConnectionFuture<T> = ConnectionFuture<T, RepoError>;

pub struct DbRepoImpl<I, F, U>
where
    I: Inserter + 'static,
    F: Filter + 'static,
    U: Updater + 'static,
{
    pub table: &'static str,
    pub insert_acl_engine: Rc<acl::AclEngine<I, RepoError>>,
    pub select_acl_engine: Rc<acl::AclEngine<F, RepoError>>,
    pub delete_acl_engine: Rc<acl::AclEngine<F, RepoError>>,
    pub update_acl_engine: Rc<acl::AclEngine<U, RepoError>>,
}

impl<I, F, U> DbRepoImpl<I, F, U>
where
    F: Filter + 'static,
    I: Inserter + 'static,
    U: Updater + 'static,
{
    pub fn new(table: &'static str) -> Self {
        Self {
            table,
            insert_acl_engine: Rc::new(acl::SystemACL),
            select_acl_engine: Rc::new(acl::SystemACL),
            delete_acl_engine: Rc::new(acl::SystemACL),
            update_acl_engine: Rc::new(acl::SystemACL),
        }
    }

    pub fn with_insert_acl_engine<E>(mut self, acl_engine: E) -> Self
    where
        E: acl::AclEngine<I, RepoError> + 'static,
    {
        self.insert_acl_engine = Rc::new(acl_engine);
        self
    }

    pub fn with_select_acl_engine<E>(mut self, acl_engine: E) -> Self
    where
        E: acl::AclEngine<F, RepoError> + 'static,
    {
        self.select_acl_engine = Rc::new(acl_engine);
        self
    }

    pub fn with_delete_acl_engine<E>(mut self, acl_engine: E) -> Self
    where
        E: acl::AclEngine<F, RepoError> + 'static,
    {
        self.delete_acl_engine = Rc::new(acl_engine);
        self
    }

    pub fn with_update_acl_engine<E>(mut self, acl_engine: E) -> Self
    where
        E: acl::AclEngine<U, RepoError> + 'static,
    {
        self.update_acl_engine = Rc::new(acl_engine);
        self
    }
}

fn query_debug(q: &str, args: &[Box<ToSql>]) -> String {
    let args_dbg = args.iter().enumerate().fold(String::new(), |mut acc, (i, arg)| {
        if i > 0 {
            acc += ", ";
        }
        acc += &format!("${} = {:?}", i + 1, arg);
        acc
    });

    format!("Query: {}. Args: {}", q, &args_dbg)
}

impl<T, I, F, U> DbRepoInsert<T, I, RepoError> for DbRepoImpl<I, F, U>
where
    F: Filter,
    T: From<Row> + 'static,
    I: Inserter,
    U: Updater,
{
    fn insert(&self, conn: RepoConnection, inserter: I) -> RepoConnectionFuture<Vec<T>> {
        let table = self.table;

        Box::new(
            self.insert_acl_engine
                .ensure_access(inserter)
                .then(move |res| {
                    future::result(match res {
                        Ok(inserter) => {
                            let (query, args) = inserter.into_insert_builder(table).build();
                            Ok((query, args, conn))
                        }
                        Err((e, _inserter)) => Err((e, conn)),
                    })
                })
                .and_then(move |(query, args, conn)| conn.prepare2(&query).map(move |(statement, conn)| (statement, query, args, conn)))
                .and_then(move |(statement, query, args, conn)| {
                    let err_msg = query_debug(&query, &args);
                    conn.query2(&statement, args)
                        .collect()
                        .map_err(move |(e, conn)| (e.context(err_msg).into(), conn))
                })
                .map(|(rows, conn)| (rows.into_iter().map(T::from).collect::<Vec<T>>(), conn))
                .map_err(|(e, conn)| (e.context("Failure while running insert").into(), conn)),
        )
    }
}

impl<T, I, F, U> DbRepoSelect<T, F, RepoError> for DbRepoImpl<I, F, U>
where
    T: From<Row> + 'static,
    F: Filter,
    I: Inserter,
    U: Updater,
{
    fn select(&self, conn: RepoConnection, filter: F) -> RepoConnectionFuture<Vec<T>> {
        let table = self.table;

        Box::new(
            self.select_acl_engine
                .ensure_access(filter)
                .then(move |res| {
                    future::result(match res {
                        Ok(filter) => {
                            let (query, args) = filter.into_filtered_operation_builder(FilteredOperation::Select, table).build();
                            Ok((query, args, conn))
                        }
                        Err((e, _filter)) => Err((e, conn)),
                    })
                })
                .and_then(move |(query, args, conn)| conn.prepare2(&query).map(move |(statement, conn)| (statement, query, args, conn)))
                .and_then(move |(statement, query, args, conn)| {
                    let err_msg = query_debug(&query, &args);
                    conn.query2(&statement, args)
                        .collect()
                        .map_err(move |(e, conn)| (e.context(err_msg).into(), conn))
                })
                .map(|(rows, conn)| (rows.into_iter().map(T::from).collect::<Vec<T>>(), conn))
                .map_err(|(e, conn)| (e.context("Failure while running select").into(), conn)),
        )
    }
}

impl<T, I, F, U> DbRepoUpdate<T, U, RepoError> for DbRepoImpl<I, F, U>
where
    T: From<Row> + 'static,
    F: Filter,
    I: Inserter,
    U: Updater,
{
    fn update(&self, conn: RepoConnection, updater: U) -> RepoConnectionFuture<Vec<T>> {
        let table = self.table;

        Box::new(
            self.update_acl_engine
                .ensure_access(updater)
                .then(move |res| {
                    future::result(match res {
                        Ok(updater) => {
                            let (query, args) = updater.into_update_builder(table).build();
                            Ok((query, args, conn))
                        }
                        Err((e, _updater)) => Err((e, conn)),
                    })
                })
                .and_then(move |(query, args, conn)| conn.prepare2(&query).map(move |(statement, conn)| (statement, query, args, conn)))
                .and_then(move |(statement, query, args, conn)| {
                    let err_msg = query_debug(&query, &args);
                    conn.query2(&statement, args)
                        .collect()
                        .map_err(move |(e, conn)| (e.context(err_msg).into(), conn))
                })
                .map(|(rows, conn)| (rows.into_iter().map(T::from).collect::<Vec<T>>(), conn))
                .map_err(|(e, conn)| (e.context("Failure while running update").into(), conn)),
        )
    }
}

impl<T, I, F, U> DbRepoDelete<T, F, RepoError> for DbRepoImpl<I, F, U>
where
    T: From<Row> + 'static,
    F: Filter,
    I: Inserter,
    U: Updater,
{
    fn delete(&self, conn: RepoConnection, filter: F) -> RepoConnectionFuture<Vec<T>> {
        let table = self.table;

        Box::new(
            self.delete_acl_engine
                .ensure_access(filter)
                .then(move |res| {
                    future::result(match res {
                        Ok(filter) => {
                            let (query, args) = filter.into_filtered_operation_builder(FilteredOperation::Delete, table).build();
                            Ok((query, args, conn))
                        }
                        Err((e, _filter)) => Err((e, conn)),
                    })
                })
                .and_then(move |(query, args, conn)| conn.prepare2(&query).map(move |(statement, conn)| (statement, query, args, conn)))
                .and_then(move |(statement, query, args, conn)| {
                    let err_msg = query_debug(&query, &args);
                    conn.query2(&statement, args)
                        .collect()
                        .map_err(move |(e, conn)| (e.context(err_msg).into(), conn))
                })
                .map(|(rows, conn)| (rows.into_iter().map(T::from).collect::<Vec<T>>(), conn))
                .map_err(|(e, conn)| (e.context("Failure while running delete").into(), conn)),
        )
    }
}

impl<T, I, F, U> DbRepo<T, I, F, U, RepoError> for DbRepoImpl<I, F, U>
where
    T: From<Row> + 'static,
    F: Filter,
    I: Inserter,
    U: Updater,
{
}
