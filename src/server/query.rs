use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use super::gql::Context;
use juniper_codegen::graphql_object;

pub struct Query;

#[graphql_object(
    context = Context,
)]
impl Query {
    fn api_version() -> &'static str {
        "1.0"
    }

    fn chainhook_store(context: &Context) -> ChainhookStore {
        ChainhookStore {
            store: context.data.clone(),
        }
    }
}

#[derive(Clone)]
pub struct ChainhookStore {
    pub store: Arc<RwLock<HashMap<u64, String>>>,
}

#[graphql_object(context = Context)]
impl ChainhookStore {
    pub fn store(&self) -> String {
        serde_json::to_string(&self.store).unwrap()
    }
}
