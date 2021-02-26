use crate::utils::{prompt, TaskType};
use std::{path::Path, process};

pub fn run(dir: &Path, opts: &crate::Opts) {
    prompt(
        TaskType::Manual,
        "Check for pending PRs: https://github.com/ember-learn/guides-source/pulls",
    );

    let (_, guides_source_dir) = crate::clone::github(dir, "ember-learn", "guides-source");

    prompt(TaskType::Automated, "Installing node dependencies");
    if !opts.dry_run {
        process::Command::new("npm")
            .current_dir(&guides_source_dir)
            .arg("install")
            .spawn()
            .expect("Could not start process")
            .wait()
            .expect("Could not install dependencies");
    }

    prompt(TaskType::Automated, "Creating new version of guides");
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

    prompt(
        TaskType::Manual,
        "Confirm new guides version is deployed before proceeding",
    );
    prompt(TaskType::Manual, "You are super duper sure it's deployed?");
    publish_algolia(&opts, &guides_source_dir);
}

/// This function runs the npm script in the project that
/// builds search index and then deploys.
fn publish_algolia(opts: &crate::Opts, dir: &std::path::PathBuf) {
    prompt(TaskType::Automated, "Publishing Algolia index");

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
