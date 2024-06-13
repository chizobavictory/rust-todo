use std::io::{self, Write};

#[derive(Debug, Clone)]
struct Todo {
    id: usize,
    title: String,
    completed: bool,
}

struct TodoList {
    todos: Vec<Todo>,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList { todos: Vec::new() }
    }

    fn create(&mut self, title: String) -> &Todo {
        let id = self.todos.len() + 1;
        let todo = Todo {
            id,
            title,
            completed: false,
        };
        self.todos.push(todo);
        self.todos.last().unwrap()
    }

    fn read(&self, id: usize) -> Option<&Todo> {
        self.todos.iter().find(|&todo| todo.id == id)
    }

    fn update(&mut self, id: usize, title: Option<String>, completed: Option<bool>) -> Option<&Todo> {
        if let Some(todo) = self.todos.iter_mut().find(|todo| todo.id == id) {
            if let Some(title) = title {
                todo.title = title;
            }
            if let Some(completed) = completed {
                todo.completed = completed;
            }
            return Some(todo);
        }
        None
    }

    fn delete(&mut self, id: usize) -> bool {
        if let Some(pos) = self.todos.iter().position(|todo| todo.id == id) {
            self.todos.remove(pos);
            return true;
        }
        false
    }

    fn list(&self) -> &Vec<Todo> {
        &self.todos
    }
}

fn main() {
    let mut todo_list = TodoList::new();
    loop {
        println!("Choose an action: [1] Create [2] Read [3] Update [4] Delete [5] List [6] Exit");
        let mut action = String::new();
        io::stdin().read_line(&mut action).expect("Failed to read line");

        match action.trim() {
            "1" => {
                print!("Enter the title of the todo: ");
                io::stdout().flush().unwrap();
                let mut title = String::new();
                io::stdin().read_line(&mut title).expect("Failed to read line");
                let todo = todo_list.create(title.trim().to_string());
                println!("Created todo: {:?}", todo);
            }
            "2" => {
                print!("Enter the ID of the todo to read: ");
                io::stdout().flush().unwrap();
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed to read line");
                let id: usize = id.trim().parse().expect("Please enter a valid number");
                match todo_list.read(id) {
                    Some(todo) => println!("Todo: {:?}", todo),
                    None => println!("Todo not found"),
                }
            }
            "3" => {
                print!("Enter the ID of the todo to update: ");
                io::stdout().flush().unwrap();
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed to read line");
                let id: usize = id.trim().parse().expect("Please enter a valid number");

                print!("Enter the new title (leave blank to keep the same): ");
                io::stdout().flush().unwrap();
                let mut title = String::new();
                io::stdin().read_line(&mut title).expect("Failed to read line");
                let title = if title.trim().is_empty() { None } else { Some(title.trim().to_string()) };

                print!("Enter the new completed status (true/false, leave blank to keep the same): ");
                io::stdout().flush().unwrap();
                let mut completed = String::new();
                io::stdin().read_line(&mut completed).expect("Failed to read line");
                let completed = if completed.trim().is_empty() { None } else { Some(completed.trim().parse().expect("Please enter true or false")) };

                match todo_list.update(id, title, completed) {
                    Some(todo) => println!("Updated todo: {:?}", todo),
                    None => println!("Todo not found"),
                }
            }
            "4" => {
                print!("Enter the ID of the todo to delete: ");
                io::stdout().flush().unwrap();
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed to read line");
                let id: usize = id.trim().parse().expect("Please enter a valid number");
                if todo_list.delete(id) {
                    println!("Deleted todo");
                } else {
                    println!("Todo not found");
                }
            }
            "5" => {
                println!("Todos: {:?}", todo_list.list());
            }
            "6" => {
                break;
            }
            _ => println!("Invalid option"),
        }
    }
}
