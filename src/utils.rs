use semver::Version;
use serde::{Deserialize, Serialize};
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

use std::fmt::Display;

pub enum TaskType {
    Automated,
    Manual,
}

impl Display for TaskType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskType::Automated => write!(f, "🤖"),
            TaskType::Manual => write!(f, "👩‍💻"),
        }
    }
}

pub fn prompt(task_type: TaskType, description: &str) {
    println!("{} {}", task_type, description);
    if let TaskType::Manual = task_type {
        crate::utils::pause();
    }
    print!("\n");
}

pub fn heroku_env_vars(project: &str) -> Vec<(String, String)> {
    prompt(TaskType::Automated, "Fetching env vars from heroku");

    let heroku_vars = std::process::Command::new("heroku")
        // .current_dir(&dir)
        .arg("config")
        .arg("-s")
        .args(&["-a", project])
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

#[derive(Serialize, Deserialize)]
struct GuidesVersionsAttributes {
    #[serde(rename = "all-versions")]
    all_versions: Vec<String>,
    #[serde(rename = "current-version")]
    current_version: String,
    #[serde(rename = "lts-versions")]
    lts_versions: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct GuidesVersionsData {
    #[serde(rename = "type")]
    data_type: String,
    id: String,
    attributes: GuidesVersionsAttributes,
}

#[derive(Serialize, Deserialize)]
struct GuidesVersions {
    data: GuidesVersionsData,
}

pub struct CurrentVersions {
    pub deployed: Version,
    pub target: Version,
}

impl CurrentVersions {
    pub fn new() -> Self {
        let versions: GuidesVersions =
            reqwest::blocking::get("https://guides.emberjs.com/content/versions.json")
                .unwrap()
                .json()
                .unwrap();
        let mut prefixed_version = versions.data.attributes.current_version.chars();
        prefixed_version.next();
        let version = prefixed_version.as_str();

        let deployed = semver::Version::parse(version).unwrap();
        let mut target = deployed.clone();
        target.increment_minor();

        Self { deployed, target }
    }
}
