use git2::Repository;
use std::{path::PathBuf, process};
use process::ExitStatus;

fn clone_repos(folder: &mut PathBuf) -> Result<Repository, git2::Error> {
    println!("🤖 Cloning ember-jsonapi-docs");
    folder.push("ember-jsonapi-docs");
    Repository::clone(
        "https://github.com/ember-learn/ember-jsonapi-docs.git",
        &folder,
    )?;
    folder.pop();

    println!("🤖 Cloning ember.js");
    folder.push("ember.js");
    Repository::clone("https://github.com/emberjs/ember.js.git", &folder)?;
    folder.pop();

    println!("🤖 Cloning data");
    folder.push("data");
    let repo = Repository::clone("https://github.com/emberjs/data.git", &folder)?;
    folder.pop();

    Ok(repo)
}

fn get_env_vars() -> Vec<(String, String)> {
    println!("🤖 Fetching env vars from heroku");
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

pub fn deploy_api_documentation() -> Result<ExitStatus, std::io::Error> {
    check_heroku_cli();

    let mut dir = tempfile::tempdir().unwrap().into_path();
    clone_repos(&mut dir).unwrap();

    println!("Installing node dependencies");
    dir.push("ember-jsonapi-docs");
    process::Command::new("yarn")
        .current_dir(&dir)
        .spawn()
        .expect("Could not install dependencies")
        .wait()?;

    println!("🤖 Generating API documentation…");
    let vars = get_env_vars();
    let result = process::Command::new("yarn")
        .current_dir(&dir)
        .envs(vars)
        .args(&["run", "start", "--sync"])
        .spawn()
        .expect("Could not compile API documentation")
        .wait();

    std::fs::remove_dir_all(dir)?;
    result
}

// Checks if heroku-cli is installed, and  then  checks if user is logged in.
fn check_heroku_cli() {
    println!("👩‍💻 Checking heroku-cli");
    match std::process::Command::new("heroku").stdout(std::process::Stdio::null()).status() {
        Ok(_) => {}
        Err(_) => {
            println!("heroku-cli not found. Please install and try again: https://devcenter.heroku.com/articles/heroku-cli");
            std::process::exit(1);
        }
    };

    match std::process::Command::new("heroku").arg("auth:whoami").status().expect("Could not confirm login").success() {
        true => {}
        false => {
            let status = std::process::Command::new("heroku").arg("login").spawn().expect("Could not log in user.").wait().expect("??");
            
            if !status.success() {
                std::process::exit(1);    
            }
        }
    };
}
