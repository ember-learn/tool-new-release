use std::process::Command;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct OpDetails {
    pub password: String,
}
#[derive(Serialize, Deserialize)]
pub struct OpItem {
    pub details: OpDetails,
}

#[derive(Serialize, Deserialize)]
pub struct OpNote {
    pub value: String,
}

fn login() {
    let status = Command::new("op")
        .stdout(std::process::Stdio::piped())
        .args(&["whoami"])
        .status()
        .unwrap();

    match status.code() {
        Some(0) => {}
        _ => {
            Command::new("op")
                .stdout(std::process::Stdio::piped())
                .args(&["signin", "--account", "ember-cli.1password.com"])
                .spawn()
                .unwrap()
                .wait_with_output()
                .unwrap();
        }
    }
}

pub fn get_glitch() -> String {
    login();
    let output = Command::new("op")
        .args(&["item", "get", "Glitch", "--fields", "password"])
        .output()
        .unwrap();
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

pub fn get_api_docs_vars() -> Vec<(String, String)> {
    login();
    let output = Command::new("op")
        .args(&[
            "item",
            "get",
            "API docs",
            "--fields",
            "notesPlain",
            "--format",
            "json",
        ])
        .output()
        .unwrap();
    let str = String::from_utf8(output.stdout).unwrap();
    let note = serde_json::from_str::<OpNote>(&str).unwrap();

    let mut res = vec![];
    for line in note.value.trim().split('\n') {
        println!("and a-one");
        let mut x = line.split('=').collect::<Vec<&str>>().into_iter();
        res.push((x.next().unwrap().to_owned(), x.next().unwrap().to_owned()));
    }

    res
}

pub fn get_guides_search_key() -> String {
    login();
    let output = Command::new("op")
        .args(&["item", "get", "Guides Search", "--fields", "password"])
        .output()
        .unwrap();
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}
