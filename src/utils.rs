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
