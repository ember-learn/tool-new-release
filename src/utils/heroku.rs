use super::prompt::automated;

pub fn get_env_vars(project: &str) -> Vec<(String, String)> {
    check_heroku_cli();

    crate::utils::prompt::automated("Fetching env vars from heroku");

    let heroku_vars = std::process::Command::new("heroku")
        .arg("config")
        .arg("-s")
        .args(&["-a", project])
        .output()
        .expect("Could not retrieve env vars.");
    let str = String::from_utf8(heroku_vars.stdout).unwrap();

    // println!("VARS\n{}", str);

    let mut res = vec![];
    for line in str.trim().split('\n') {
        let mut x = line.split('=').collect::<Vec<&str>>().into_iter();
        res.push((x.next().unwrap().to_owned(), x.next().unwrap().to_owned()));
    }

    res
}

// Checks if heroku-cli is installed, and  then  checks if user is logged in.
fn check_heroku_cli() {
    automated("Checking heroku-cli");

    if heroku_cmd()
        .stdout(std::process::Stdio::null())
        .status()
        .is_err()
    {
        eprintln!("heroku-cli not found. Please install and try again: https://devcenter.heroku.com/articles/heroku-cli");
        std::process::exit(1);
    }

    if !heroku_cmd()
        .arg("auth:whoami")
        .status()
        .expect("Could not confirm login")
        .success()
    {
        let status = heroku_cmd()
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

#[cfg(windows)]
fn heroku_cmd() -> std::process::Command {
    let mut cmd = std::process::Command::new("cmd");
    cmd.args(&["/C", "heroku"]);
    cmd
}

#[cfg(not(windows))]
fn heroku_cmd() -> std::process::Command {
    std::process::Command::new("heroku")
}