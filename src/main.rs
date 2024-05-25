use std::{
    env,
    fmt::Debug,
    fs,
    time::{Duration, Instant},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Task {
    task_id: u64,
    description: String,
    time_spent: u64,
}

impl Debug for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\nTask {{ \n task_id: {},\n description: {},\n time_spent: {}\n }}",
            self.task_id, self.description, self.time_spent
        )
    }
}

fn main() {
    let contents: String = match fs::read_to_string("./tasks.json") {
        Ok(contents) => contents,
        Err(_) => {
            fs::write("./tasks.json", "").unwrap();
            String::from("[]")
        }
    };

    let mut tasks = serde_json::from_str::<Vec<Task>>(contents.as_str()).expect("Can't read file");

    let args: Vec<String> = env::args().collect();
    let command = &args[1].to_lowercase();
    // make sure the second argument is a string
    let text = &args[2];

    if command == "add" {
        let task = Task {
            task_id: 1,
            time_spent: 0,
            description: text.clone(),
        };
        tasks.push(task);
        let serialized_tasks = serde_json::to_string(&tasks).expect("Failed to serialize tasks");
        fs::write("./tasks.json", serialized_tasks).unwrap();
    }
}
