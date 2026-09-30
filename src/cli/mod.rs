mod utils;

pub fn cli() {
    println!("===== SIMPLE TODO CLI =====");

    let _tasks: Vec<utils::Task> = utils::_read().expect("Failed to load tasks file!");

    loop {
        println!("\n=====     OPTIONS     =====");
        println!("1. Add");
        println!("2. Update");
        println!("3. remove");
        println!("4. List");
        println!("5. ShutDown");

        let choice = utils::read_input("Choose an option: ");

        let choice: u32 = match choice.parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        match choice {
            1 => {
                let _task = utils::read_input("Task: ");
                let task = utils::Task::new(_task);
                utils::write(task).unwrap();
            }
    
        2 => {
            let task_id = utils::read_input("Task(id) to Update: ")
                .parse::<u64>()
                .unwrap();

            println!("1. Incomplete");
            println!("2. InProgress");
            println!("3. Completed");

            let action: String = utils::read_input("What Status You wanna change into? ");

            match action.parse::<u64>() {
                Ok(num) => {
                    let result = match num {
                        1 => utils::_update(task_id, |task| {
                            task.status = utils::Status::Incomplete;
                        }),
                        2 => utils::_update(task_id, |task| {
                            task.status = utils::Status::InProgress;
                        }),
                        3 => utils::_update(task_id, |task| {
                            task.status = utils::Status::Completed;
                        }),
                        _ => {
                            println!("Invalid status.");
                            return;
                        }
                    };

                    if let Err(e) = result {
                        println!("Error updating task: {:?}", e);
                    }
                }

                Err(_) => {
                    println!("Please enter a valid number.");
                }
            }
        }

            3 => {
                let mut _task = utils::read_input("Task to remove: ")
                    .parse::<u64>()
                    .unwrap();
                
                let _ = utils::_delete(_task);
                println!("The {_task} is deleted");
            }

            4 => {
                let tasks = utils::_read();
                match tasks {
                    Ok(task) => {
                        println!("\n=================== TASKS ===================");
                        println!("{:<5} | {:<12} | {}", "ID", "Status", "Title");
                        println!("----------------------------------------------");
                        
                        for t in task {
                            let status_str = format!("{:?}", t.status);
                            println!("{:<5} | {:<12} | {}", t.id, status_str, t.title);
                        }
                        println!("=============================================");
                    }
                    Err(e) => {
                        println!("Error loading tasks: {}", e);
                    }
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
