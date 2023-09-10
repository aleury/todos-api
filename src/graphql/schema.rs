use async_graphql::{EmptyMutation, EmptySubscription, MergedObject, Schema, SchemaBuilder};

use super::todos::TodoQuery;

#[derive(MergedObject, Default)]
pub struct QueryRoot(TodoQuery);

pub fn build() -> SchemaBuilder<QueryRoot, EmptyMutation, EmptySubscription> {
    Schema::build(QueryRoot::default(), EmptyMutation, EmptySubscription)
}
