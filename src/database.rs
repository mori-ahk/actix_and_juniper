use crate::schema::Todo;
use std::collections::HashMap;

#[derive(Clone, Default)]
pub struct Database {
    // pool: ConnectionPool,
    todos: HashMap<i32, Todo>,
}

impl Database {
    pub fn new() -> Self {
        let mut todos = HashMap::new();
        for i in 0..5 {
            let title = format!("todo {}", i);
            let todo = Todo::new(i, title.to_string());
            todos.insert(todo.id, todo);
        }

        Self { todos }
    }

    pub fn all_todos(&self) -> Vec<Todo> {
        self.todos.values().cloned().collect()
    }

    pub fn get_todo(&self, id: i32) -> Option<&Todo> {
        self.todos.get(&id)
    }
}

impl juniper::Context for Database {}
