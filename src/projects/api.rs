use crate::{Opts, utils::{prompt, TaskType}};
use std::{path::PathBuf, process};

pub fn run(mut dir: &mut PathBuf, opts: &Opts) {
    println!("Beginning deploy for: API Documentation\n");

    if cfg!(windows) {
        check_heroku_cli_windows();
    } else {
        check_heroku_cli();
    }

    crate::repo::Repo {
        organization: "ember-learn",
        project: "ember-jsonapi-docs",
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
    if opts.dry_run {
        process::Command::new("yarn")
            .current_dir(&dir)
            .arg("install")
            .spawn()
            .expect("Could not spawn new process")
            .wait()
            .expect("Could not install dependencies");
    }

    prompt(TaskType::Automated, "Generating API documentation…");
    let vars = crate::utils::heroku_env_vars("api-viewer-json-docs-generator");
        process::Command::new("yarn")
            .current_dir(&dir)
            .envs(vars)
            .args(&["run", "start", "--sync"])
            .spawn()
            .unwrap()
            .wait()
            .expect("Could not compile API documentation");
    }
    dir.pop();

    clean_temporary_files(&dir);
}

// Here we try to clean the cloned repositories.
// Since we are operating inside a temporary directory,
// we don't consider failure to remove the direcotry to be fatal.
fn clean_temporary_files(dir: &PathBuf) {
    if let Err(_) = std::fs::remove_dir_all(dir) {}
}

// Checks if heroku-cli is installed, and  then  checks if user is logged in.
// I was getting bogged down on building up the command according to the platform, so...
fn check_heroku_cli_windows() {
    prompt(TaskType::Automated, "Checking heroku-cli");

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
    prompt(TaskType::Automated, "Checking heroku-cli");

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
