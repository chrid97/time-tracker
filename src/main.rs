use std::{
    error::Error,
    io::{stdout, Write},
    process,
};
use chrono::Local;
use tokio::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let start_time = Local::now().format("%I:%M:%S");
    let timer = Instant::now();
    tokio::spawn(async move {
        loop {
            print!("\r{}", format_seconds(timer.elapsed().as_secs()));
            stdout().flush().unwrap();
        }
    });
    println!("Time Started: {}", start_time);

    tokio::signal::ctrl_c().await?;
    let end_time = Local::now().format("%I:%M:%S");
    println!("\nTime Ended: {}", end_time);
    println!("Total Time Elapsed: {}", format_seconds(timer.elapsed().as_secs()));
    process::exit(0);
}

fn format_seconds(secs: u64) -> String {
    format!("{:02}:{:02}", secs / 60, secs % 60)
}
