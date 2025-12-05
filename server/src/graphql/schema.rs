use async_graphql::{EmptySubscription, Schema};

use super::mutation::MutationRoot;
use super::query::QueryRoot;

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;
