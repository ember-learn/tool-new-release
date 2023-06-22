use crate::{utils::prompt::automated};
use std::{process};

pub fn run(dir: &std::path::Path, dry_run: bool) {
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
            .envs(vars)
            .args(&["run", "start", "--sync"])
            .spawn()
            .unwrap()
            .wait()
            .expect("Could not compile API documentation");
    }
}
