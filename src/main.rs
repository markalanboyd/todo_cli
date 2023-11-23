use::std::io::{self, Read, Write};
use::std::fs::{File, OpenOptions};
use::serde_json;
use::serde::{Serialize, Deserialize};
use::std::collections::HashMap;
use colored::Colorize;

fn main() {
    loop {
        println!();
        println!("todo_cli");
        println!("----------------------------");
        println!("{} Add task", "[A]".green());
        println!("{} View tasks", "[V]".green());
        println!("{} Mark a task as completed", "[M]".green());
        println!("{} Edit a task", "[E]".green());
        println!("{} Delete a task", "[D]".green());
        println!("{} View completed tasks", "[S]".green());
        println!("{} Quit", "[Q]".red());

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read line.");
        let user_input_lowercase = user_input.trim().to_lowercase();

        match user_input_lowercase.as_str() {
            "a" => add_task(),
            "q" => break,
            _ => println!("Invalid command, try again.\n"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    description: String,
}

type TaskList = HashMap<u32, Task>;

pub fn add_task() {
    let mut task_description = String::new();
    println!("Add task: ");
    io::stdin().read_line(&mut task_description).expect("Failed to read line.");
    let task = Task {
        description: task_description.trim().to_string(),
    };
    let mut tasks = load_tasks();
    let next_id = tasks.keys().max().unwrap_or(&0) + 1;
    tasks.insert(next_id, task);
    save_tasks(&tasks);
    view_tasks(&tasks);
}

pub fn load_tasks() -> TaskList {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("tasks.json")
        .expect("Failed to open tasks file.");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read tasks file.");
    return if contents.is_empty() {
        TaskList::new()
    } else {
        serde_json::from_str(&contents).expect("Failed to deserialize tasks.")
    }
}

pub fn save_tasks(tasks: &TaskList) {
    let serialized = serde_json::to_string(tasks).expect("Failed to serialize tasks.");
    let mut file = File::create("tasks.json").expect("Failed to create tasks file.");
    file.write_all(serialized.as_bytes()).expect("Failed to write tasks file.");
}

pub fn view_tasks(tasks: &TaskList) {
    for (id, task) in tasks {
        println!("{}: {}", id, task.description);
    }
}
