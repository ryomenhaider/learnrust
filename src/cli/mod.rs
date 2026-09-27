use std::collections::HashMap;
use std::io;

pub fn cli () {
    println!("===== SIMPLE TODO CLI =====");
    loop {
        println!("=====     OPTIONS     =====");
        println!("1. Add");
        println!("2. Remove");
        println!("3. Update");
        println!("4. List");
        println!("5. ShutDown");

        let mut tasks = HashMap::new();
        let mut input = String::new();
        let default_task_state = "incomplete";
        let mut task: String = String::new();

            
        io::stdin()
        .read_line(&mut input)
        .expect("Failed to Read Line");
        
        let choice: u32 = input.trim().parse().expect("Please type a number!");
        
        if choice == 1 {
            println!("What Task You Want to Add?");
            println!("> ");
            
            io::stdin()
            .read_line(&mut task)
            .expect("Failed to Read Line");
            
            if tasks.contains_key(&task){
                println!("{} already exist", task);
            }
            else {
                tasks.insert(task.clone(), default_task_state);
                println!("{} is added in tasks", task);
            }
        }

        else if choice == 2 {
            println!("What Task You Want to Remove?");
            println!("> ");
            
            io::stdin()
            .read_line(&mut task)
            .expect("Failed to Read Line");
            
            if tasks.contains_key(&task) {
                tasks.remove(&task);
                println!("{} is deleted", task)
            }
            else {
                println!("{} does not exist", task)
            }
        }

        else if choice == 3{
            println!("What Task You Want to Update?");
            println!("> ");
            
            io::stdin()
            .read_line(&mut task)
            .expect("Failed to Read Line");

            if tasks.contains_key(&task) {
                tasks.insert(task.clone(), default_task_state);
                println!("{} is added in tasks", task);
            }
            else {
                println!("{} does not exist", task)
            }
        }
        else if choice == 4 {
            println!("Task   |   Status");
            for (key, value) in &tasks {
               println!("{:<5} | {}", key, value); 
            }
        }
        else if choice == 5 {
            break;
        }
    }
}