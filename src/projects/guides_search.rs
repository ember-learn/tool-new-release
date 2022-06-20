use crate::utils::prompt::{automated, manual};
use std::{path::Path, process};

pub fn run(dir: &Path, opts: &crate::Opts) {
    manual("Confirm new guides version is deployed before proceeding");
    manual("You are super duper sure it's deployed?");

    automated("Publishing Algolia index");

    if !opts.dry_run {
        process::Command::new("npm")
            .current_dir(&dir)
            .arg("run")
            .arg("release:search")
            .spawn()
            .expect("Couldn't start process.")
            .wait()
            .expect("Failed to publish algolia index");
    }
}
