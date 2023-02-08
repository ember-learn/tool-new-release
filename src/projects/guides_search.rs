use crate::utils::prompt::{automated, manual};
use std::{path::Path, process};

pub fn run(dir: &Path, opts: &crate::Opts) {
    let (_, guides_source_dir) = crate::clone::github(dir, "ember-learn", "guides-source");

    automated("Publishing Algolia index");

    if !opts.dry_run {
        process::Command::new("npm")
            .current_dir(&guides_source_dir)
            .arg("run")
            .arg("release:search")
            .spawn()
            .expect("Couldn't start process.")
            .wait()
            .expect("Failed to publish algolia index");
    }
}
