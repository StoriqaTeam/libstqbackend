//! This crate provides common ACL facilities, namely the common groups and traits.

extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;

use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;

pub type DbConnection = PgConnection;

/// Implement this trait on resource to signal if it's in the current scope
pub trait WithScope<Scope> {
    fn is_in_scope(&self, scope: &Scope, user_id: i32, conn: Option<&DbConnection>) -> bool;
}

/// Access control layer for repos. It tells if a user can do a certain action with
/// certain resource. All logic for roles and permissions should be hardcoded into implementation
/// of this trait.
pub trait Acl<Resource, Action, Scope, Error> {
    /// Tells if a user with id `user_id` can do `action` on `resource`.
    /// `resource_with_scope` can tell if this resource is in some scope, which is also a part of `acl` for some
    /// permissions. E.g. You can say that a user can do `Create` (`Action`) on `Store` (`Resource`) only if he's the
    /// `Owner` (`Scope`) of the store.
    fn allows(
        &self,
        resource: &Resource,
        action: &Action,
        resources_with_scope: &[&WithScope<Scope>],
        conn: Option<&DbConnection>,
    ) -> Result<bool, Error>;
}

/// `SystemACL` allows all manipulation with resources in all cases.
#[derive(Clone, Debug, Default)]
pub struct SystemACL {}

#[allow(unused)]
impl<Resource, Action, Scope, Error> Acl<Resource, Action, Scope, Error> for SystemACL {
    fn allows(
        &self,
        resource: &Resource,
        action: &Action,
        resources_with_scope: &[&WithScope<Scope>],
        conn: Option<&DbConnection>,
    ) -> Result<bool, Error> {
        Ok(true)
    }
}

/// `UnauthorizedACL` denies all manipulation with resources in all cases.
#[derive(Clone, Debug, Default)]
pub struct UnauthorizedACL {}

#[allow(unused)]
impl<Resource, Action, Scope, Error> Acl<Resource, Action, Scope, Error> for UnauthorizedACL {
    fn allows(
        &self,
        resource: &Resource,
        action: &Action,
        resources_with_scope: &[&WithScope<Scope>],
        conn: Option<&DbConnection>,
    ) -> Result<bool, Error> {
        Ok(false)
    }
}

pub trait RolesCache {
    type Error;
    type Role;

    fn get(&self, id: i32, db_conn: Option<&DbConnection>) -> Result<Vec<Self::Role>, Self::Error>;
    fn clear(&self) -> Result<(), Self::Error>;
    fn remove(&self, id: i32) -> Result<(), Self::Error>;
}
