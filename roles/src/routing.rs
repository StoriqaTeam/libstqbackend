use models::*;
use service::*;

use failure;
use futures::future;
use futures::prelude::*;
use hyper::{Body, Delete, Get, Method, Post};
use serde::{de::DeserializeOwned, Serialize};
use std::rc::Rc;
use stq_http::request_util::*;
use stq_router::RouteParser;
use stq_types::*;

#[derive(Clone, Debug)]
pub enum Route {
    Roles,
    RoleById(RoleEntryId),
    RolesByUserId(UserId),
}

pub fn add_routes<R>(mut route_parser: RouteParser<R>) -> RouteParser<R>
where
    R: From<Route>,
{
    route_parser.add_route(r"^/roles$", || Route::Roles.into());
    route_parser.add_route_with_params(r"^/roles/by-user-id/(\d+)$", |params| {
        params
            .get(0)
            .and_then(|string_id| string_id.parse().ok())
            .map(|v| Route::RolesByUserId(v).into())
    });
    route_parser.add_route_with_params(r"^/roles/by-id/(\S+)$", |params| {
        params
            .get(0)
            .and_then(|string_id| string_id.parse().ok())
            .map(|v| Route::RoleById(v).into())
    });

    route_parser
}

pub struct Controller<T> {
    pub service: Rc<RoleService<T>>,
}

impl<T> Controller<T>
where
    T: Serialize + DeserializeOwned + 'static,
{
    pub fn call(&self, method: Method, route: Route, payload: Body) -> Box<Future<Item = String, Error = failure::Error>> {
        let service = self.service.clone();

        match (method, route) {
            (Get, Route::RolesByUserId(user_id)) => serialize_future({ service.get_roles_for_user(user_id) }),
            (Post, Route::Roles) => {
                serialize_future({ parse_body::<RoleEntry<T>>(payload).and_then(move |data| service.create_role(data)) })
            }
            (Delete, Route::RolesByUserId(user_id)) => serialize_future({
                parse_body::<Option<T>>(payload).and_then(move |role| service.remove_role(RoleSearchTerms::Meta((user_id, role))))
            }),
            (Delete, Route::RoleById(role_id)) => serialize_future({ service.remove_role(RoleSearchTerms::Id(role_id)) }),
            // Fallback
            (other_method, other_route) => Box::new(future::err(format_err!(
                "Could not route request {} {:?}",
                other_method,
                other_route
            ))),
        }
    }
}
