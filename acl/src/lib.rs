//! This crate provides common ACL facilities, namely the common groups and traits.
#[macro_use]
extern crate failure;
extern crate futures;

use futures::future;
use futures::prelude::*;

/// Implement this trait on resource to signal if it's in the current scope
pub trait CheckScope<Scope, T> {
    fn is_in_scope(&self, user_id: i32, scope: &Scope, obj: Option<&T>) -> bool;
}

pub type Verdict<E> = Box<Future<Item = bool, Error = E> + Send>;

#[derive(Clone, Debug, Fail)]
#[fail(display = "Unauthorized")]
pub struct UnauthorizedError;

/// Access control layer for repos. It tells if a user can do a certain action with
/// certain resource. All logic for roles and permissions should be hardcoded into implementation
/// of this trait.
pub trait AclEngine<Context, Error>: Send + Sync
where
    Error: Send + From<UnauthorizedError> + 'static,
{
    /// Tells if a user with id `user_id` can do `action` on `resource`.
    /// `resource_with_scope` can tell if this resource is in some scope, which is also a part of `acl` for some
    /// permissions. E.g. You can say that a user can do `Create` (`Action`) on `Store` (`Resource`) only if he's the
    /// `Owner` (`Scope`) of the store.
    fn allows(&self, ctx: Context) -> Verdict<Error>;

    fn ensure_access(&self, ctx: Context) -> Box<Future<Item = (), Error = Error> + Send> {
        Box::new(self.allows(ctx).and_then(|allowed| {
            future::result(match allowed {
                true => Ok(()),
                false => Err(Error::from(UnauthorizedError)),
            })
        }))
    }
}

impl<Context, Error, T> AclEngine<Context, Error> for T
where
    Error: Send + From<UnauthorizedError> + 'static,
    T: Fn(Context) -> Verdict<Error> + Send + Sync,
{
    fn allows(&self, ctx: Context) -> Verdict<Error> {
        (self)(ctx)
    }
}

/// `SystemACL` allows all manipulation with resources in all cases.
#[derive(Clone, Debug, Default)]
pub struct SystemACL;

#[allow(unused)]
impl<Context, Error> AclEngine<Context, Error> for SystemACL
where
    Error: Send + From<UnauthorizedError> + 'static,
{
    fn allows(&self, _ctx: Context) -> Verdict<Error> {
        Box::new(future::ok((true)))
    }
}

/// `UnauthorizedACL` denies all manipulation with resources in all cases.
#[derive(Clone, Debug, Default)]
pub struct UnauthorizedACL;

#[allow(unused)]
impl<Context, Error> AclEngine<Context, Error> for UnauthorizedACL
where
    Error: Send + From<UnauthorizedError> + 'static,
{
    fn allows(&self, _ctx: Context) -> Verdict<Error> {
        Box::new(future::ok((false)))
    }
}

pub trait RolesCache: Clone + Send + 'static {
    type Role;

    fn get(&self, user_id: i32) -> Vec<Self::Role>;
    fn clear(&self);
    fn remove(&self, user_id: i32);
    fn contains(&self, user_id: i32) -> bool;
    fn add_roles(&self, user_id: i32, roles: &Vec<Self::Role>);
}
