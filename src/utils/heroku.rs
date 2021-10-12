use super::prompt::automated;

pub fn get_env_vars(project: &str) -> Vec<(String, String)> {
    if cfg!(windows) {
        check_heroku_cli_windows();
    } else {
        check_heroku_cli();
    }

    crate::utils::prompt::automated("Fetching env vars from heroku");

    let heroku_vars = std::process::Command::new("heroku")
        // .current_dir(&dir)
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
// I was getting bogged down on building up the command according to the platform, so...
fn check_heroku_cli_windows() {
    automated("Checking heroku-cli");

    if std::process::Command::new("cmd")
        .args(&["/C", "heroku"])
        .stdout(std::process::Stdio::null())
        .status()
        .is_err()
    {
        eprintln!("heroku-cli not found. Please install and try again: https://devcenter.heroku.com/articles/heroku-cli");
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
        eprintln!("heroku-cli not found. Please install and try again: https://devcenter.heroku.com/articles/heroku-cli");
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
