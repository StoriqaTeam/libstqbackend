use models::*;
use repo::*;

use failure;
use futures::future;
use futures::prelude::*;
use std::fmt::Debug;
use std::rc::Rc;
use stq_db::pool::Pool as DbPool;
use stq_db::repo::*;
use stq_types::*;

pub type ServiceFuture<T> = Box<Future<Item = T, Error = failure::Error>>;

pub trait RoleService<T> {
    fn get_roles_for_user(&self, user_id: UserId) -> ServiceFuture<Vec<RoleEntry<T>>>;
    fn create_role(&self, item: RoleEntry<T>) -> ServiceFuture<RoleEntry<T>>;
    fn remove_role(&self, filter: RoleSearchTerms<T>) -> ServiceFuture<Option<RoleEntry<T>>>;
    fn remove_all_roles(&self, user_id: UserId) -> ServiceFuture<Vec<RoleEntry<T>>>;
}

pub struct RoleServiceImpl<T> {
    pub repo_factory: Rc<Fn() -> Box<RolesRepo<T>>>,
    pub role_faucet: Rc<Fn() -> Vec<RoleEntry<T>>>,
    pub db_pool: DbPool,
}

impl<T> RoleServiceImpl<T>
where
    T: RoleModel + Clone,
{
    pub fn new(db_pool: DbPool, caller_id: Option<UserId>) -> Box<Future<Item = Self, Error = failure::Error>> {
        match caller_id {
            None => Box::new(future::ok(Self {
                db_pool,
                role_faucet: Rc::new(|| vec![]),
                repo_factory: Rc::new(|| Box::new(make_repo(RepoLogin::Anonymous))),
            })),
            Some(caller_id) => Box::new(
                db_pool
                    .run(move |conn| make_su_repo().select(conn, RoleSearchTerms::Meta((caller_id, None)).into()))
                    .map_err(|e| e.context("Failed to fetch user roles").into())
                    .map(move |caller_roles| {
                        let db_pool = db_pool.clone();
                        Self {
                            db_pool,
                            role_faucet: Rc::new({
                                let caller_roles = caller_roles.clone();
                                move || caller_roles.clone()
                            }),
                            repo_factory: Rc::new(move || {
                                Box::new(make_repo(RepoLogin::User {
                                    caller_id,
                                    caller_roles: caller_roles.clone(),
                                }))
                            }),
                        }
                    }),
            ),
        }
    }
}

impl<T> RoleService<T> for RoleServiceImpl<T>
where
    T: RoleModel + Clone + Debug,
{
    fn get_roles_for_user(&self, user_id: UserId) -> ServiceFuture<Vec<RoleEntry<T>>> {
        let repo_factory = self.repo_factory.clone();
        Box::new(
            self.db_pool
                .run(move |conn| (repo_factory)().select(conn, RoleSearchTerms::Meta((user_id, None)).into()))
                .map_err(move |e| e.context(format!("Failed to get roles for user {}", user_id.0)).into()),
        )
    }
    fn create_role(&self, item: RoleEntry<T>) -> ServiceFuture<RoleEntry<T>> {
        let repo_factory = self.repo_factory.clone();
        Box::new(
            self.db_pool
                .run({
                    let item = item.clone();
                    move |conn| (repo_factory)().insert_exactly_one(conn, item)
                })
                .map_err(move |e| e.context(format!("Failed to create role: {:?}", item)).into()),
        )
    }
    fn remove_role(&self, filter: RoleSearchTerms<T>) -> ServiceFuture<Option<RoleEntry<T>>> {
        let repo_factory = self.repo_factory.clone();
        Box::new(
            self.db_pool
                .run({
                    let filter = filter.clone();
                    move |conn| (repo_factory)().delete(conn, filter.into())
                })
                .map(|mut v| v.pop())
                .map_err(move |e| e.context(format!("Failed to remove role: {:?}", filter)).into()),
        )
    }
    fn remove_all_roles(&self, user_id: UserId) -> ServiceFuture<Vec<RoleEntry<T>>> {
        let repo_factory = self.repo_factory.clone();
        Box::new(
            self.db_pool
                .run(move |conn| (repo_factory)().delete(conn, RoleSearchTerms::Meta((user_id, None)).into()))
                .map_err(move |e| e.context(format!("Failed to remove all roles for user {}", user_id.0)).into()),
        )
    }
}
