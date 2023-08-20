use std::io::{self, Write};

struct Todo {
    tasks: Vec<String>,
}

impl Todo {
    fn new() -> Todo {
        Todo { tasks: Vec::new() }
    }

    fn add_task(&mut self, task: String) {
        self.tasks.push(task);
    }

    fn remove_task(&mut self, index: usize) {
        if let Some(_) = self.tasks.get(index) {
            self.tasks.remove(index);
        }
    }

    fn complete_task(&mut self, index: usize) {
        if let Some(task) = self.tasks.get_mut(index) {
            task.push_str(" (Completed)");
        }
    }

    fn print_tasks(&self) {
        println!("Tasks:");
        for (index, task) in self.tasks.iter().enumerate() {
            println!("{}: {}", index, task);
        }
    }
}

fn main() {
    let mut todo = Todo::new();

    loop {
        println!("Menu:");
        println!("1. Add Task");
        println!("2. Remove Task");
        println!("3. Mark Task as Completed");
        println!("4. Print Tasks");
        println!("5. Quit");

        print!("Select an option: ");
        io::stdout().flush().unwrap();

        let mut option = String::new();
        io::stdin().read_line(&mut option).unwrap();

        match option.trim().parse::<u32>() {
            Ok(1) => {
                print!("Enter task description: ");
                io::stdout().flush().unwrap();

                let mut task = String::new();
                io::stdin().read_line(&mut task).unwrap();

                todo.add_task(task.trim().to_string());
                println!("Task added successfully!");
            }
            Ok(2) => {
                todo.print_tasks();

                print!("Enter task number to remove: ");
                io::stdout().flush().unwrap();

                let mut index = String::new();
                io::stdin().read_line(&mut index).unwrap();

                match index.trim().parse::<usize>() {
                    Ok(i) => {
                        todo.remove_task(i);
                        println!("Task removed successfully!");
                    }
                    Err(_) => {
                        println!("Invalid task number!");
                    }
                }
            }
            Ok(3) => {
                todo.print_tasks();

                print!("Enter task number to mark as completed: ");
                io::stdout().flush().unwrap();

                let mut index = String::new();
                io::stdin().read_line(&mut index).unwrap();

                match index.trim().parse::<usize>() {
                    Ok(i) => {
                        todo.complete_task(i);
                        println!("Task marked as completed!");
                    }
                    Err(_) => {
                        println!("Invalid task number!");
                    }
                }
            }
            Ok(4) => {
                todo.print_tasks();
            }
            Ok(5) => {
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid option!");
            }
        }
    }
}