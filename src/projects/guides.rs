use crate::utils::{prompt, TaskType};
use process::ExitStatus;
use std::{path::PathBuf, process};

pub fn deploy(mut dir: &mut PathBuf) -> Result<ExitStatus, std::io::Error> {
    println!("Beginning deploy for: Guides\n");
    prompt(
        TaskType::Manual,
        "Check for pending PRs: https://github.com/ember-learn/guides-source/pulls",
    );

    crate::repo::Repo {
        organization: "ember-learn",
        project: "guides-source",
    }
    .clone(&mut dir);

    prompt(TaskType::Automated, "Installing node dependencies");
    dir.push("guides-source");
    process::Command::new("npm")
        .current_dir(&dir)
        .arg("install")
        .spawn()
        .expect("Could not install dependencies")
        .wait()?;

    prompt(TaskType::Automated, "Creating new version of guides");
    process::Command::new("npm")
        .current_dir(&dir)
        .arg("run")
        .arg("release:guides:minor")
        .spawn()
        .expect("Failed to release guides.")
        .wait()?;

    prompt(
        TaskType::Manual,
        "Confirm new guides version is deployed before proceeding",
    );
    prompt(TaskType::Manual, "You are super duper sure it's deployed?");
    prompt(TaskType::Automated, "Publishing Algolia index");

    process::Command::new("npm")
        .arg("run")
        .arg("release:search")
        .spawn()
        .expect("Failed to publish algolia index.")
        .wait()?;

    std::process::exit(0)
}
