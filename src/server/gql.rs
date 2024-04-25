use super::query::Query;
use juniper::{EmptyMutation, EmptySubscription, RootNode};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

pub struct Context {
    pub data: Arc<RwLock<HashMap<u64, String>>>,
}

impl juniper::Context for Context {}

pub type NestorGraphqlSchema =
    RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;

pub fn new_graphql_schema() -> NestorGraphqlSchema {
    NestorGraphqlSchema::new(Query, EmptyMutation::new(), EmptySubscription::new())
}
