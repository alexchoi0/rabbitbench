use async_graphql::{EmptySubscription, Schema};

use super::query::QueryRoot;
use super::mutation::MutationRoot;

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;
