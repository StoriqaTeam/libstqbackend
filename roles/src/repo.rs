use models::*;

use failure;
use futures::future;
use stq_acl::*;
use stq_db::repo::*;
use stq_db::statement::{UpdateBuilder, Updater};
use stq_types::*;

const TABLE: &'static str = "roles";

pub struct DummyRoleUpdater;

impl Updater for DummyRoleUpdater {
    fn into_update_builder(self, _table: &'static str) -> UpdateBuilder {
        unreachable!()
    }
}

pub trait RolesRepo<T>: DbRepo<RoleEntry<T>, RoleEntry<T>, RoleFilter<T>, DummyRoleUpdater, RepoError>
where
    T: RoleModel,
{
}

pub type RolesRepoImpl<T> = DbRepoImpl<RoleEntry<T>, RoleEntry<T>, RoleFilter<T>, DummyRoleUpdater>;
impl<T> RolesRepo<T> for RolesRepoImpl<T>
where
    T: RoleModel,
{
}

pub fn make_su_repo<T>() -> RolesRepoImpl<T>
where
    T: RoleModel,
{
    RolesRepoImpl::new(TABLE)
}

fn check_acl<T>(
    caller_id: UserId,
    caller_roles: Vec<RoleEntry<T>>,
    entry: RoleEntry<T>,
    action: Action,
) -> Verdict<(RoleEntry<T>, Action), failure::Error>
where
    T: RoleModel,
{
    let mut res = false;

    for user_role in caller_roles.into_iter() {
        // Superadmins can do anything.
        if user_role.role.is_su() {
            res = true;
            break;
        }
    }

    // Others can only view their roles.
    if action == Action::Select {
        res = caller_id == entry.user_id;
    }

    Box::new(future::ok((res, (entry, action))))
}

pub enum RepoLogin<T> {
    Anonymous,
    User {
        caller_id: UserId,
        caller_roles: Vec<RoleEntry<T>>,
    },
}

/// Creates roles repo. No access for anonymous users, sorry.
pub fn make_repo<T>(login: RepoLogin<T>) -> RolesRepoImpl<T>
where
    T: RoleModel + Clone,
{
    use self::RepoLogin::*;

    let repo = make_su_repo();

    match login {
        Anonymous => repo.with_afterop_acl_engine(ForbiddenACL),
        User { caller_id, caller_roles } => {
            repo.with_afterop_acl_engine({ move |(entry, action)| check_acl(caller_id, caller_roles.clone(), entry, action) })
        }
    }
}
