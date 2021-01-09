use std::io;
use std::io::prelude::*;

#[allow(dead_code)]
pub fn read_input(message: &str) -> String {
    print!("{} ", message);
    let _ = std::io::stdout().flush();

    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);
    buffer
}

#[allow(dead_code)]
pub fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

use std::fmt::Display;

pub enum TaskType {
    Automated,
    Manual,
}

impl Display for TaskType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskType::Automated => write!(f, "🤖"),
            TaskType::Manual => write!(f, "👩‍💻"),
        }
    }
}

pub fn prompt(task_type: TaskType, description: &str) {
    println!("{} {}", task_type, description);
    if let TaskType::Manual = task_type {
        crate::utils::pause();
    }
    print!("\n");
}

pub fn heroku_env_vars(project: &str) -> Vec<(String, String)> {
    prompt(TaskType::Automated, "Fetching env vars from heroku");

    let heroku_vars = std::process::Command::new("heroku")
        // .current_dir(&dir)
        .arg("config")
        .arg("-s")
        .args(&["-a", project])
        .output()
        .expect("Could not retrieve env vars.");
    let str = String::from_utf8(heroku_vars.stdout.to_owned()).unwrap();

    let mut res = vec![];
    for line in str.trim().split("\n") {
        let mut x = line.split("=").collect::<Vec<&str>>().into_iter();
        res.push((x.next().unwrap().to_owned(), x.next().unwrap().to_owned()));
    }

    res
}
