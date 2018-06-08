//! This crate provides common ACL facilities, namely the common groups and traits.
extern crate failure;
extern crate futures;

use futures::future;
use futures::prelude::*;
use std::fmt;
use std::fmt::{Debug, Display};

/// Implement this trait on resource to signal if it's in the current scope
pub trait CheckScope<Scope, T> {
    fn is_in_scope(&self, user_id: i32, scope: &Scope, obj: Option<&T>) -> bool;
}

pub type Verdict<Context, E> = Box<Future<Item = (bool, Context), Error = E> + Send>;
#[derive(Debug)]
pub struct UnauthorizedError<Context: Debug + Send + Sync + 'static>(Context);

impl<Context> Display for UnauthorizedError<Context>
where
    Context: Debug + Send + Sync,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unauthorized: {:?}", self.0)
    }
}

impl<Context> std::error::Error for UnauthorizedError<Context>
where
    Context: Debug + Send + Sync,
{
    fn description(&self) -> &str {
        "unauthorized"
    }
}

/// Access control layer for repos. It tells if a user can do a certain action with
/// certain resource. All logic for roles and permissions should be hardcoded into implementation
/// of this trait.
pub trait AclEngine<Context, Error>: Send + Sync
where
    Context: Debug + Send + Sync + 'static,
    Error: Send + From<UnauthorizedError<Context>> + 'static,
{
    /// Tells if a user with id `user_id` can do `action` on `resource`.
    /// `resource_with_scope` can tell if this resource is in some scope, which is also a part of `acl` for some
    /// permissions. E.g. You can say that a user can do `Create` (`Action`) on `Store` (`Resource`) only if he's the
    /// `Owner` (`Scope`) of the store.
    fn allows(&self, ctx: Context) -> Verdict<Context, Error>;

    fn ensure_access(&self, ctx: Context) -> Box<Future<Item = Context, Error = Error> + Send> {
        Box::new(self.allows(ctx).and_then(|(allowed, _ctx)| {
            if allowed {
                Box::new(future::ok(_ctx))
            } else {
                Box::new(future::err(Error::from(UnauthorizedError(_ctx))))
            }
        }))
    }
}

/// `SystemACL` allows all manipulation with resources in all cases.
#[derive(Clone, Debug, Default)]
pub struct SystemACL {}

#[allow(unused)]
impl<Context, Error> AclEngine<Context, Error> for SystemACL
where
    Context: Debug + Send + Sync + 'static,
    Error: Send + From<UnauthorizedError<Context>> + 'static,
{
    fn allows(&self, _ctx: Context) -> Verdict<Context, Error> {
        Box::new(future::ok((true, _ctx)))
    }
}

/// `UnauthorizedACL` denies all manipulation with resources in all cases.
#[derive(Clone, Debug, Default)]
pub struct UnauthorizedACL {}

#[allow(unused)]
impl<Context, Error> AclEngine<Context, Error> for UnauthorizedACL
where
    Context: Debug + Send + Sync + 'static,
    Error: Send + From<UnauthorizedError<Context>> + 'static,
{
    fn allows(&self, _ctx: Context) -> Verdict<Context, Error> {
        Box::new(future::ok((false, _ctx)))
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
