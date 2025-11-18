// Project: 📬 Command Line Todo App
// 	•	Each todo item: Pending, InProgress, Done (enum).
// 	•	Match user commands (add, remove, list, mark_done).
// 	•	Save/load from JSON file.
// Builds toward: Real CLI workflows, error handling, enums.

use std::io;

#[derive(Debug)]
enum Status {
    Pending,
    // InProgress,
    // Done,
}

#[derive(Debug)]
struct TodoItem{
    title: String,
    // description: String,
    status: Status,
}

struct Todo{
    items: Vec<TodoItem>
}

impl Todo{
    fn add(&mut self){
        println!("____Add todo:");
        println!("Enter an item to add to your todo list:");
        let mut title = String::new();
        io::stdin().read_line(&mut title).expect("failed to read item");
        let item = TodoItem{
            title: title.trim().to_string(),
            status: Status::Pending
        };
        println!("{:?}", item);
        self.items.push(item);
    }
}

fn main() {
    println!("_____A6 Command Line Todo App");

    let mut todo = Todo{items: Vec::new()};
    println!("Enter your command: [add, remove, list, inprogress, done]");

    let mut command = String::new();
    io::stdin().read_line(&mut command).expect("failed to read command");
    println!("You entered: {}", command);
    match command.to_lowercase().trim() {
        "add" => todo.add(),
        "remove" => println!("remove todo:"),
        "list" => println!("listing todo:"),
        _ => println!("error: invalid command")
    }
}
