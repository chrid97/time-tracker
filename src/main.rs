use serde::{Deserialize, Serialize};
use std::{
    env,
    fmt::Debug,
    fs,
    io::{self, stdout, Write},
    process,
    time::{Instant, SystemTime},
};

#[derive(Serialize, Deserialize)]
struct Task {
    name: String,
    time: Vec<StartAndStop>,
}

#[derive(Serialize, Deserialize)]
struct StartAndStop {
    start_time: SystemTime,
    end_time: SystemTime,
}

// impl Debug for Task {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "\nTask {{ \n name: {},\n time_spent: {}\n }}",
//             self.name, self.time_spent
//         )
//     }
// }

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
            name: text.clone(),
            time: vec![],
        };
        tasks.push(task);
        let serialized_tasks = serde_json::to_string(&tasks).expect("Failed to serialize tasks");
        fs::write("./tasks.json", serialized_tasks).unwrap();
    }

    if command == "timer" {
        let err_message = format!("Failed to find task name {}", text);
        let mut active_task = tasks
            .iter_mut()
            .find(|task| task.name == text.to_owned())
            .expect(&err_message);

        let start_time = SystemTime::now();
        let timer = Instant::now();
        loop {
            print!("\r{}", secs_to_minutes(timer.elapsed().as_secs()));
            stdout().flush().unwrap();
        }
        // let end_time = SystemTime::now();
        // active_task.time.push(StartAndStop {
        //     start_time,
        //     end_time,
        // })
    }
}

// maybe replace u64 with Duration type
fn secs_to_minutes(seconds: u64) -> String {
    let minutes = seconds / 60;
    let seconds = seconds % 60;
    format!("{:02}:{:02}", minutes, seconds)
}
