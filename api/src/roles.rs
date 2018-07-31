use rpc_client::RpcClientImpl;
use types::*;
use util::*;

use failure;
use futures::future;
use futures::prelude::*;
use reqwest::unstable::async::RequestBuilder;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;
use stq_roles::models::*;
use stq_roles::routing::*;
use stq_types::UserId;

impl RouteBuilder for Route {
    fn route(&self) -> String {
        match self {
            Route::Roles => "roles".into(),
            Route::RoleById(entry_id) => format!("roles/by-id/{}", entry_id),
            Route::RolesByUserId(user_id) => format!("roles/by-user-id/{}", user_id),
        }
    }
}

pub trait RolesClient<T>
where
    T: RoleModel + Clone + Debug + Serialize + DeserializeOwned + Send,
{
    fn get_roles_for_user(&self, user_id: UserId) -> ApiFuture<T>;
    fn create_role(&self, item: RoleEntry<T>) -> ApiFuture<RoleEntry<T>>;
    fn remove_role(&self, terms: RoleSearchTerms<T>) -> ApiFuture<Option<RoleEntry<T>>>;
}

impl<T> RolesClient<T> for RpcClientImpl
where
    T: RoleModel + Clone + Debug + Serialize + DeserializeOwned + Send,
{
    fn get_roles_for_user(&self, user_id: UserId) -> ApiFuture<T> {
        http_req(
            self.http_client
                .get(&self.build_route(&Route::RolesByUserId(user_id))),
        )
    }

    fn create_role(&self, item: RoleEntry<T>) -> ApiFuture<RoleEntry<T>> {
        let http_client = self.http_client.clone();
        let route = self.build_route(&Route::Roles);
        Box::new(
            serialize_payload(item)
                .and_then(move |body| http_req(http_client.post(&route).body(body))),
        )
    }

    fn remove_role(&self, terms: RoleSearchTerms<T>) -> ApiFuture<Option<RoleEntry<T>>> {
        let http_client = self.http_client.clone();
        Box::new({
            let fut = match terms {
                RoleSearchTerms::Id(id) => Box::new(future::ok(
                    http_client.delete(&self.build_route(&Route::RoleById(id))),
                ))
                    as Box<Future<Item = RequestBuilder, Error = failure::Error> + Send>,
                RoleSearchTerms::Meta((user_id, entry)) => {
                    let route = self.build_route(&Route::RolesByUserId(user_id));
                    Box::new(serialize_payload(entry).map(move |body| {
                        let mut req = http_client.delete(&route);
                        req.body(body);
                        req
                    }))
                }
            };

            fut.and_then(|mut req| req.send().map_err(failure::Error::from))
                .and_then(|mut rsp| rsp.json().map_err(failure::Error::from))
        })
    }
}
