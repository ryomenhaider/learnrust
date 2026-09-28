use serde::{Serialize, Deserialize};
use chrono::{DateTime, Local};
use std::io::{self, Write, BufReader, BufRead};
use std::fs::{OpenOptions, File};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);


#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u32,
    title: String,
    status: Status,
    created_at: DateTime<Local>,
}

#[derive(Serialize, Deserialize, Debug)]
enum Status {
    Incomplete,
    InProgress,
    Completed,
}

impl Task {
    fn new (title: String) -> Self {
        let uniq = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        Task {
            id: uniq,
            title,
            status: Status::Incomplete,
            created_at: Local::now(),
        }
    }
}

fn write(message: Task) -> std::io::Result<()>
{
    let task :Task = Task {

        id: message.id,
        title: message.title,
        status: message.status,
        created_at: Local::now()
    };
    let mut files = OpenOptions::new()
        .append(true)
        .create(true)
        .open("tasks.json")?;
    
    let json_byte = serde_json::to_vec(&task).map_err(
        |e| std::io::Error::new(std::io::ErrorKind::InvalidData, e)
    )?;

    files.write_all(&json_byte)?;
    files.write_all(b"\n");

    Ok(())
}

fn _read() -> std::io::Result<Vec<Task>> 
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



fn read_input(message: &str) -> String {
    let mut input = String::new();

    print!("{}", message);
    io::stdout().flush().expect("Failed to flush stdout");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}