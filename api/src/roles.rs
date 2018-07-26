use rpc_client::RpcClientImpl;
use util::serialize_payload;

use failure;
use futures::future;
use futures::prelude::*;
use reqwest::unstable::async::RequestBuilder;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;
use stq_roles::models::*;
use stq_roles::routing::*;
use stq_types::UserId;

pub type ClientFuture<T> = Box<Future<Item = T, Error = failure::Error> + Send>;

pub trait RolesClient<T>
where
    T: RoleModel + Clone + Debug + Serialize + DeserializeOwned + Send,
{
    fn get_roles_for_user(&self, user_id: UserId) -> ClientFuture<T>;
    fn create_role(&self, item: RoleEntry<T>) -> ClientFuture<RoleEntry<T>>;
    fn remove_role(&self, terms: RoleSearchTerms<T>) -> ClientFuture<Option<RoleEntry<T>>>;
}

impl<T> RolesClient<T> for RpcClientImpl
where
    T: RoleModel + Clone + Debug + Serialize + DeserializeOwned + Send,
{
    fn get_roles_for_user(&self, user_id: UserId) -> ClientFuture<T> {
        Box::new(
            self.http_client
                .get(&Route::RolesByUserId(user_id).route())
                .send()
                .and_then(|mut rsp| rsp.json())
                .map_err(failure::Error::from),
        )
    }

    fn create_role(&self, item: RoleEntry<T>) -> ClientFuture<RoleEntry<T>> {
        let http_client = self.http_client.clone();
        Box::new(
            serialize_payload(item)
                .and_then(move |body| {
                    http_client
                        .post(&Route::Roles.route())
                        .body(body)
                        .send()
                        .map_err(failure::Error::from)
                })
                .and_then(|mut rsp| rsp.json().map_err(failure::Error::from)),
        )
    }

    fn remove_role(&self, terms: RoleSearchTerms<T>) -> ClientFuture<Option<RoleEntry<T>>> {
        let http_client = self.http_client.clone();
        Box::new({
            let fut = match terms {
                RoleSearchTerms::Id(id) => {
                    Box::new(future::ok(http_client.delete(&Route::RoleById(id).route())))
                        as Box<Future<Item = RequestBuilder, Error = failure::Error> + Send>
                }
                RoleSearchTerms::Meta((user_id, entry)) => {
                    Box::new(serialize_payload(entry).map(move |body| {
                        let mut req = http_client.delete(&Route::RolesByUserId(user_id).route());
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
