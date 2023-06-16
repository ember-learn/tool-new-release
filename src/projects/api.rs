use crate::{utils::prompt::automated};
use std::{process};
use regex::Regex;

pub fn run(dir: &std::path::Path, dry_run: bool, new_version: String) {
    let re = Regex::new(r"v\d+\.\d+\.\d+").unwrap();

    if !re.is_match(&new_version) {
        panic!("you need to provide --new-version in the format v4.11.0 (including the v)");
    }

    let vars = crate::utils::op::api_docs::read();
    crate::utils::clone::github(dir, "ember-learn", "ember-jsonapi-docs");

    automated("Installing node dependencies");
    if !dry_run {
        process::Command::new("yarn")
            .current_dir(dir)
            .arg("install")
            .spawn()
            .expect("Could not spawn new process")
            .wait()
            .expect("Could not install dependencies");
    }

    automated("Generating API documentation…");
    if !dry_run {
        process::Command::new("yarn")
            .current_dir(dir)
            .envs(vars.iter())
            .args(&["run", "start", "--sync"])
            .spawn()
            .unwrap()
            .wait()
            .expect("Could not compile API documentation");
    }

    automated(format!("Uploading ember api docs for {}", new_version).as_str());
    if !dry_run {
        process::Command::new("yarn")
            .current_dir(dir)
            .envs(vars.iter())
            .args(&["run", "start", "--project", "ember", "--version", new_version.as_str(), "--ignorePreviouslyIndexedDoc"])
            .spawn()
            .unwrap()
            .wait()
            .expect("Could not upload ember api docs API documentation");
    }

    automated(format!("Uploading ember-data api docs for {}", new_version).as_str());
    if !dry_run {
        process::Command::new("yarn")
            .current_dir(dir)
            .envs(vars)
            .args(&["run", "start", "--project", "ember-data", "--version", new_version.as_str(), "--ignorePreviouslyIndexedDoc"])
            .spawn()
            .unwrap()
            .wait()
            .expect("Could not upload ember api docs API documentation");
    }
}
