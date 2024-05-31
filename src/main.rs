use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use std::{
    env,
    error::Error,
    fs,
    io::{stdout, Write},
    process,
};
use tokio::time::Instant;

#[derive(Serialize, Deserialize)]
struct Project {
    name: String,
    total_time: usize,
    hours_worked: Vec<HoursWorked>,
}

#[derive(Serialize, Deserialize)]
struct HoursWorked {
    start_time: DateTime<Local>,
    end_time: DateTime<Local>,
    time_elapsed: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // probably change projects to a dictionary/hashmap the key = project name
    let contents = match fs::read_to_string("tasks.json") {
        Ok(contents) => contents,
        Err(_) => {
            fs::write("tasks.json", "[]").unwrap();
            String::from("[]")
        }
    };
    let mut projects =
        serde_json::from_str::<Vec<Project>>(contents.as_str()).expect("Can't read file");
    let args: Vec<String> = env::args().collect();
    let command = args.iter().nth(1).expect("Wrong command");

    // ig commands can be a hashmap, key = command: value = function
    if command == "new" {
        let name = args.iter().nth(2).expect("Project needs a name");
        let project = Project {
            name: name.clone(),
            total_time: 0,
            hours_worked: vec![],
        };
        projects.push(project);
        let serialized_tasks = serde_json::to_string(&projects).expect("Failed to serialize tasks");
        fs::write("./tasks.json", serialized_tasks).unwrap();
    }

    if command == "timer" {
        let hours_works = start_timer().await?;
        // (TODO) save change to file
        match args.iter().nth(2) {
            Some(name) => projects
                .iter_mut()
                .find(|project| project.name == name.to_owned())
                .expect("Failed to find project")
                .hours_worked
                .push(hours_works),
            None => (),
        };
        process::exit(0);
    }
    Ok(())
}

async fn start_timer() -> Result<HoursWorked, Box<dyn Error>> {
    let start_time = Local::now();
    let timer = Instant::now();
    tokio::spawn(async move {
        loop {
            print!("\r{}", format_seconds(timer.elapsed().as_secs()));
            stdout().flush().unwrap();
        }
    });
    println!("Time Started: {}", start_time.format("%I:%M:%S"));

    tokio::signal::ctrl_c().await?;
    let end_time = Local::now();
    println!("\nTime Ended: {}", end_time.format("%I:%M:%S"));
    println!(
        "Total Time Elapsed: {}",
        format_seconds(timer.elapsed().as_secs())
    );

    Ok(HoursWorked {
        start_time,
        end_time,
        time_elapsed: timer.elapsed().as_secs(),
    })
    // process::exit(0);
}

fn format_seconds(secs: u64) -> String {
    format!("{:02}:{:02}", secs / 60, secs % 60)
}
