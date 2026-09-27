use std::collections::HashMap;
use std::io::{self, Write};

pub fn cli() {
    println!("===== SIMPLE TODO CLI =====");

    let mut tasks: HashMap<String, String> = HashMap::new();

    loop {
        println!("\n=====     OPTIONS     =====");
        println!("1. Add");
        println!("2. Remove");
        println!("3. Update");
        println!("4. List");
        println!("5. ShutDown");

        let choice = read_input("Choose an option: ");

        let choice: u32 = match choice.parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        match choice {
            1 => {
                let task = read_input("Task: ");

                if task.is_empty() {
                    println!("Task cannot be empty!");
                    continue;
                }

                if tasks.contains_key(&task) {
                    println!("\"{}\" already exists.", task);
                } else {
                    tasks.insert(task.clone(), String::from("incomplete"));
                    println!("\"{}\" was added to your tasks.", task);
                }
            }

            2 => {
                let task = read_input("Task to remove: ");

                if tasks.remove(&task).is_some() {
                    println!("\"{}\" was deleted.", task);
                } else {
                    println!("\"{}\" does not exist.", task);
                }
            }

            3 => {
                let task = read_input("Task to update: ");

                if tasks.contains_key(&task) {
                    println!("Choose new status:");
                    println!("1. Incomplete");
                    println!("2. In Progress");
                    println!("3. Complete");

                    let status_choice = read_input("Status: ");

                    let new_status = match status_choice.parse::<u32>() {
                        Ok(1) => "incomplete",
                        Ok(2) => "in progress",
                        Ok(3) => "complete",
                        _ => {
                            println!("Invalid status!");
                            continue;
                        }
                    };

                    tasks.insert(task.clone(), new_status.to_string());

                    println!(
                        "\"{}\" was updated to \"{}\".",
                        task, new_status
                    );
                } else {
                    println!("\"{}\" does not exist.", task);
                }
            }

            4 => {
                if tasks.is_empty() {
                    println!("No tasks found.");
                    continue;
                }

                println!("\n===== TASKS =====");
                println!("{:<30} | Status", "Task");
                println!("-----------------------------------------------");

                for (task, status) in &tasks {
                    println!("{:<30} | {}", task, status);
                }
            }
            5 => {
                println!("Shutting down...");
                break;
            }
            _ => {
                println!("Invalid option! Please choose 1-5.");
            }
        }
    }
}


fn read_input(message: &str) -> String {
    let mut input = String::new();

    print!("{}", message);
    io::stdout().flush().expect("Failed to flush stdout");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}
