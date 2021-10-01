use crate::utils::prompt::automated;
use crate::Opts;
use std::process;

pub fn run(dir: &std::path::Path, opts: &Opts) {
    if cfg!(windows) {
        check_heroku_cli_windows();
    } else {
        check_heroku_cli();
    }

    let (_, jsonapi_docs_dir) = crate::clone::github(dir, "ember-learn", "ember-jsonapi-docs");

    automated("Installing node dependencies");
    if !opts.dry_run {
        process::Command::new("yarn")
            .current_dir(&jsonapi_docs_dir)
            .arg("install")
            .spawn()
            .expect("Could not spawn new process")
            .wait()
            .expect("Could not install dependencies");
    }

    automated("Generating API documentation…");
    let vars = crate::utils::heroku::get_env_vars("api-viewer-json-docs-generator");
    if !opts.dry_run {
        process::Command::new("yarn")
            .current_dir(&jsonapi_docs_dir)
            .envs(vars)
            .args(&["run", "start", "--sync"])
            .spawn()
            .unwrap()
            .wait()
            .expect("Could not compile API documentation");
    }
}

// Checks if heroku-cli is installed, and  then  checks if user is logged in.
// I was getting bogged down on building up the command according to the platform, so...
fn check_heroku_cli_windows() {
    automated("Checking heroku-cli");

    if std::process::Command::new("cmd")
        .args(&["/C", "heroku"])
        .stdout(std::process::Stdio::null())
        .status()
        .is_err()
    {
        println!("heroku-cli not found. Please install and try again: https://devcenter.heroku.com/articles/heroku-cli");
        std::process::exit(1);
    }

    if !std::process::Command::new("cmd")
        .args(&["/C", "heroku", "auth:whoami"])
        .status()
        .expect("Could not confirm login")
        .success()
    {
        let status = std::process::Command::new("cmd")
            .args(&["/C", "heroku", "login"])
            .spawn()
            .expect("Could not log in user.")
            .wait()
            .expect("??");

        if !status.success() {
            std::process::exit(1);
        }
    }
}

// Checks if heroku-cli is installed, and  then  checks if user is logged in.
fn check_heroku_cli() {
    automated("Checking heroku-cli");

    if std::process::Command::new("heroku")
        .stdout(std::process::Stdio::null())
        .status()
        .is_err()
    {
        println!("heroku-cli not found. Please install and try again: https://devcenter.heroku.com/articles/heroku-cli");
        std::process::exit(1);
    }

    if !std::process::Command::new("heroku")
        .arg("auth:whoami")
        .status()
        .expect("Could not confirm login")
        .success()
    {
        let status = std::process::Command::new("heroku")
            .arg("login")
            .spawn()
            .expect("Could not log in user.")
            .wait()
            .expect("??");

        if !status.success() {
            std::process::exit(1);
        }
    }
}
