fn is_installed() {
    let status = op()
        .stdout(std::process::Stdio::piped())
        .args(&["whoami"])
        .status();

    if status.is_err() {
        install();
    }
}

#[cfg(target_os = "macos")]
fn install() {
    use super::prompt::automated;

    automated("Could not find op binary. Installing using homebrew…");
    std::process::Command::new("brew")
        .args(&[
            "install",
            "--quiet",
            "--cask",
            "1password/tap/1password-cli",
        ])
        .spawn()
        .expect("Couldn't execute Homebrew.")
        .wait_with_output()
        .expect("Homebrew failed.");
}

#[cfg(not(target_os = "macos"))]
fn install() {
    eprintln!("Could not find op binary. Follow the installation instructions at https://1password.com/downloads/command-line/ and try running this tool again.");
    std::process::exit(-1);
}

fn is_logged_in() {
    let status = op()
        .stdout(std::process::Stdio::piped())
        .args(&["whoami"])
        .status()
        .expect("Could not retrieve 1Password account details");

    match status.code() {
        Some(0) => {}
        _ => {
            op().stdout(std::process::Stdio::piped())
                .args(&["signin", "--account", "ember-cli.1password.com"])
                .spawn()
                .expect("Could not start 1password-cli")
                .wait_with_output()
                .expect("Could not log in");
        }
    }
}

fn check_1password_cli() {
    is_installed();
    is_logged_in();
}

fn op() -> std::process::Command {
    std::process::Command::new("op")
}

pub fn read(path: &str) -> String {
    check_1password_cli();

    let output = op().args(&["read", path]).output().unwrap();
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}
pub mod glitch {
    pub fn read() -> String {
        super::read("op://Ember Learning Team/Glitch/password")
    }
}

pub mod api_docs {
    use std::collections::HashMap;

    type AwsCredentials = HashMap<String, String>;

    pub fn read() -> AwsCredentials {
        let read = super::read("op://Ember Learning Team/api_docs_toml/notesPlain");
        toml::from_str::<AwsCredentials>(&read).unwrap()
    }
}
