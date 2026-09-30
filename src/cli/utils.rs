use serde::{Serialize, Deserialize};
use std::io::{self, Write, BufReader, BufRead};
use std::fs::{OpenOptions, File};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: u64,
    pub title: String,
    pub status: Status,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Status {
    Incomplete,
    InProgress,
    Completed,
}

impl Task {
    pub fn new (title: String) -> Self {
        let uniq: u64 = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        Task {
            id: uniq,
            title,
            status: Status::Incomplete,
        }
    }
}

pub fn write(message: Task) -> std::io::Result<()>
{
    let task :Task = Task {

        id: message.id,
        title: message.title,
        status: message.status,
    };
    let mut files = OpenOptions::new()
        .append(true)
        .create(true)
        .open("tasks.json")?;
    
    let json_byte = serde_json::to_vec(&task).map_err(
        |e| std::io::Error::new(std::io::ErrorKind::InvalidData, e)
    )?;

    files.write_all(&json_byte)?;
    files.write_all(b"\n")?;

    Ok(())
}

pub fn _read() -> std::io::Result<Vec<Task>> 
{
    let file = File::open("tasks.json")?;

    let reader = BufReader::new(file);
    let mut tasks = Vec::new();

    for line in reader.lines() {
        let line_content = line?;
        
        if line_content.trim().is_empty() {
            continue;
        }
        
        let task = serde_json::from_str(&line_content).map_err(
            |e| std::io::Error::new(std::io::ErrorKind::InvalidData, e)
        
        )?;

        tasks.push(task);
    }   

    Ok(tasks)
}

pub fn _update<F: FnOnce(&mut Task)>(task_id: u64, update_fn: F) -> std::io::Result<()> {
    let mut tasks: Vec<Task> = _read()?;

    let target_task = tasks.iter_mut().find(|t| t.id == task_id);
    
    match target_task {
        Some(task) => update_fn(task),
        None => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound, 
                format!("{task_id} not found")));
        }
    }

    let updated_json = serde_json::to_string_pretty(&tasks)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    std::fs::write("tasks.json", updated_json)?;
    
    Ok(())
}

pub fn _delete(task_id: u64) -> std::io::Result<()>{
    let mut tasks = _read()?;
    let original_len = tasks.len();
    
    tasks.retain(|task| task.id != task_id);
    if tasks.len() == original_len {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Task with ID {} not found", task_id),
        ));
    }
    let updated_json = serde_json::to_string_pretty(&tasks)
        .map_err(|e| std::io::Error::new(
            std::io::ErrorKind::NotFound, e))?;

    std::fs::write("tasks.json", updated_json)?;
    Ok(())
}

pub fn read_input(message: &str) -> String {
    let mut input = String::new();

    print!("{}", message);
    io::stdout().flush().expect("Failed to flush stdout");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}