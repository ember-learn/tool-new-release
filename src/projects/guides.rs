use crate::utils::{prompt, TaskType};
use std::{path::PathBuf, process};

pub fn deploy(mut dir: &mut PathBuf, opts: crate::Opts) {
    println!("Beginning deploy for: Guides\n");

    prompt(
        TaskType::Manual,
        "Check for pending PRs: https://github.com/ember-learn/guides-source/pulls",
    );

    if !opts.dry_run {
        crate::repo::Repo {
            organization: "ember-learn",
            project: "guides-source",
        }
        .clone(&mut dir);
    }

    prompt(TaskType::Automated, "Installing node dependencies");
    dir.push("guides-source");
    if !opts.dry_run {
        process::Command::new("npm")
            .current_dir(&dir)
            .arg("install")
            .spawn()
            .expect("Could not start process")
            .wait()
            .expect("Could not install dependencies");
    }

    prompt(TaskType::Automated, "Creating new version of guides");
    if !opts.dry_run {
        process::Command::new("npm")
            .current_dir(&dir)
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
    publish_algolia(&opts);
}

/// This function runs the npm script in the project that
/// builds search index and then deploys.
fn publish_algolia(opts: &crate::Opts) {
    prompt(TaskType::Automated, "Publishing Algolia index");

    if !opts.dry_run {
        process::Command::new("npm")
            .arg("run")
            .arg("release:search")
            .spawn()
            .expect("Couldn't start process.")
            .wait()
            .expect("Failed to publish algolia index");
    }
}
