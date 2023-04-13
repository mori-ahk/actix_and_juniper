use juniper::{graphql_object, GraphQLObject};
use juniper::EmptySubscription;
pub use crate::database::Database;

pub type Schema = juniper::RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Database>>;

#[derive(Clone, GraphQLObject)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub is_done: bool,
}

impl Todo {
    pub fn new(id: i32, title: String) -> Self {
        Self {
            id,
            title,
            is_done: false,
        }
    }
}

pub struct QueryRoot;

#[graphql_object(context = Database)]
impl QueryRoot {
    fn api_version() -> &'static str {
        "0.1"
    }

    fn todo(context: &Database, id: i32) -> Option<&Todo> {
        context.get_todo(id)
    }

    fn get_all_todos(context: &Database) -> Vec<Todo> {
        context.all_todos()
    }
}

pub struct MutationRoot;

#[graphql_object(context = Database)]
impl MutationRoot {
    fn create_todo(id: i32, title: String) -> Option<Todo> {
        let todo = Todo::new(id, title);
        Some(todo)
    }
}

pub fn schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot, EmptySubscription::<Database>::new())
}
