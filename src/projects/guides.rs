use crate::utils::prompt::{automated, manual};
use std::{path::Path, process};

pub fn run(dir: &Path, opts: &crate::Opts) {
    manual("Check for pending PRs: https://github.com/ember-learn/guides-source/pulls");

    let (_, guides_source_dir) = crate::clone::github(dir, "ember-learn", "guides-source");

    automated("Installing node dependencies");
    if !opts.dry_run {
        process::Command::new("npm")
            .current_dir(&guides_source_dir)
            .arg("install")
            .spawn()
            .expect("Could not start process")
            .wait()
            .expect("Could not install dependencies");
    }

    automated("Creating new version of guides");
    if !opts.dry_run {
        process::Command::new("npm")
            .current_dir(&guides_source_dir)
            .arg("run")
            .arg("release:guides:minor")
            .spawn()
            .expect("Could not start process")
            .wait()
            .expect("Failed to release guides.");
    }

    manual("Confirm new guides version is deployed before proceeding");
    manual("You are super duper sure it's deployed?");
    publish_algolia(opts, &guides_source_dir);
}

/// This function runs the npm script in the project that
/// builds search index and then deploys.
fn publish_algolia(opts: &crate::Opts, dir: &std::path::Path) {
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
