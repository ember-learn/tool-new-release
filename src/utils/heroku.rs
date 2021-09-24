pub fn get_env_vars(project: &str) -> Vec<(String, String)> {
    crate::utils::prompt::prompt(
        crate::utils::prompt::TaskType::Automated,
        "Fetching env vars from heroku",
    );

    let heroku_vars = std::process::Command::new("heroku")
        // .current_dir(&dir)
        .arg("config")
        .arg("-s")
        .args(&["-a", project])
        .output()
        .expect("Could not retrieve env vars.");
    let str = String::from_utf8(heroku_vars.stdout).unwrap();

    let mut res = vec![];
    for line in str.trim().split('\n') {
        let mut x = line.split('=').collect::<Vec<&str>>().into_iter();
        res.push((x.next().unwrap().to_owned(), x.next().unwrap().to_owned()));
    }

    res
}
