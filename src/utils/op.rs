fn is_installed() {
    let status = op()
        .stdout(std::process::Stdio::piped())
        .args(&["whoami"])
        .status();

    if let Err(_) = status {
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
        .status();

    if let Ok(exit_status) = status {
        match exit_status.code() {
            Some(0) => {}
            _ => {
                let login = op()
                    .stdout(std::process::Stdio::piped())
                    .args(&["signin", "--account", "ember-cli.1password.com"])
                    .spawn();

                match login {
                    Ok(result) => {
                        if let Ok(res) = result.wait_with_output() {
                        } else {
                            eprintln!("Oops.");
                        }
                    }
                    Err(err) => {
                        eprintln!("Could not log in: {}", err);
                    }
                }
            }
        }
    } else {
        eprintln!("op-cli is not installed.")
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

    pub type AwsCredentials = HashMap<String, String>;

    pub fn read() -> AwsCredentials {
        let read = super::read("op://Ember Learning Team/api_docs_toml/notesPlain");
        toml::from_str::<AwsCredentials>(&read).unwrap()
    }
}
