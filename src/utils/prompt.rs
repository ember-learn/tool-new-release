use std::{fmt::Display, io::Write};

pub fn read_input(message: &str) -> String {
    print!("{} ", message);
    let _ = std::io::stdout().flush();

    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);
    buffer
}

pub fn pause() {
    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = std::io::Read::read(&mut stdin, &mut [0u8]).unwrap();
}
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
        pause();
    }
    println!();
}
