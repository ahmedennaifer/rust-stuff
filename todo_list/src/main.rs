use std::{fmt::Debug, io};

#[derive(Debug)]
struct Todo {
    name: String,
    category: String,
    expected_duration: f32,
    completed: bool,
}

#[derive(Debug)]
struct TodoList {
    name: String,
    todos: Vec<Todo>,
    id: i32,
}

impl TodoList {
    pub fn new(name: String) -> Self {
        TodoList {
            name,
            todos: Vec::new(),
            id: 1,
        }
    }
    pub fn get_todo(&self, name: &str) -> Option<&Todo> {
        self.todos.iter().find(|todo| todo.name == name)
    }
}

fn insert_todo(todo: &mut TodoList) {
    let mut inputs = Vec::new();
    let mut task = Todo {
        name: String::new(),
        category: String::new(),
        expected_duration: 0.0,
        completed: false,
    };
    let mut input = String::new();
    println!("Please Enter name, category, expected duration ");
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading input!");

    inputs = input.split_whitespace().map(String::from).collect();

    if inputs.len() < 3 {
        eprintln!("Not enough inputs provided!");
        return;
    }

    task = Todo {
        name: inputs[0].to_string(),
        category: inputs[1].to_string(),
        expected_duration: inputs[2].parse().unwrap(),
        completed: false,
    };

    todo.todos.push(task);
    println!("Todo inserted!");
}

fn main() {
    let mut todo = TodoList::new(String::from("Tests"));
    loop {
        println!("\nAvailable commands: add, find, quit");
        println!("Enter your command:");

        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read command");
        let command = command.trim();

        match command {
            "add" => insert_todo(&mut todo),
            "find" => {
                println!("Enter the name of the todo to find:");
                let mut name = String::new();
                io::stdin()
                    .read_line(&mut name)
                    .expect("Failed to read name");
                let name = name.trim();

                match todo.get_todo(name) {
                    Some(todo) => println!("Found todo: {:?}", todo),
                    None => eprintln!("Todo not found!"),
                }
            }
            "quit" => {
                println!("Exiting. Goodbye!");
                break;
            }
            _ => eprintln!("Unknown instruction"),
        }
    }
}
