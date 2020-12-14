use crate::utils::{prompt, TaskType};
use process::ExitStatus;
use std::{path::PathBuf, process};

pub fn deploy(mut dir: &mut PathBuf) {
    println!("Beginning deploy for: API Documentation\n");

    if cfg!(windows) {
        check_heroku_cli_windows();
    } else {
        check_heroku_cli();
    }

    crate::repo::Repo {
        organization: "ember-learn",
        project: "guides-source",
    }
    .clone(&mut dir);
    crate::repo::Repo {
        organization: "emberjs",
        project: "ember.js",
    }
    .clone(&mut dir);
    crate::repo::Repo {
        organization: "emberjs",
        project: "data",
    }
    .clone(&mut dir);

    prompt(TaskType::Automated, "Installing node dependencies");
    dir.push("ember-jsonapi-docs");
    process::Command::new("yarn")
        .current_dir(&dir)
        .spawn()
        .expect("Could not spawn new process")
        .wait()
        .expect("Could not install dependencies");

    prompt(TaskType::Automated, "Generating API documentation…");
    let vars = get_env_vars();
    process::Command::new("yarn")
        .current_dir(&dir)
        .envs(vars)
        .args(&["run", "start", "--sync"])
        .spawn()
        .expect("Could not spawn new process")
        .wait()
        .expect("Could not compile API documentation");

    clean_temporary_files(&dir);
}

// Here we try to clean the cloned repositories.
// Since we are operating inside a temporary directory,
// we don't consider failure to remove the direcotry to be fatal.
fn clean_temporary_files(dir: &PathBuf) {
    if let Err(_) = std::fs::remove_dir_all(dir) {}
}

fn get_env_vars() -> Vec<(String, String)> {
    prompt(TaskType::Automated, "Fetching env vars from heroku");

    let heroku_vars = std::process::Command::new("heroku")
        // .current_dir(&dir)
        .arg("config")
        .arg("-s")
        .args(&["-a", "api-viewer-json-docs-generator"])
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

// Checks if heroku-cli is installed, and  then  checks if user is logged in.
// I was getting bogged down on building up the command according to the platform, so...
fn check_heroku_cli_windows() {
    prompt(TaskType::Manual, "Checking heroku-cli");

    if let Err(_) = std::process::Command::new("cmd")
        .args(&["/C", "heroku"])
        .stdout(std::process::Stdio::null())
        .status()
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
    prompt(TaskType::Manual, "Checking heroku-cli");

    if let Err(_) = std::process::Command::new("heroku")
        .stdout(std::process::Stdio::null())
        .status()
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
