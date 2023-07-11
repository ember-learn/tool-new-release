use crate::utils::prompt::{automated, manual};
use std::{path::Path, process};
use semver::{Version, VersionReq};

pub fn run(dir: &Path, dry_run: bool) {
    let npm_version_command = process::Command::new("node")
            .arg("--version")
            .output()
            .expect("Could not check node version");

    let stdout_result = String::from_utf8(npm_version_command.stdout).unwrap();

    let req = VersionReq::parse(">=14.0.0, <15.0.0").unwrap();

    let version = Version::parse(&stdout_result[1..stdout_result.len()].trim()).unwrap();

    if  !req.matches(&version) {
        panic!("Guides can only be installed with node version 14 right now. you have {:?}", stdout_result)
    }

    manual("Check for pending PRs: https://github.com/ember-learn/guides-source/pulls");

    crate::utils::clone::github(dir, "ember-learn", "guides-source");

    automated("Installing node dependencies");
    if !dry_run {
        let status = process::Command::new("npm")
            .current_dir(dir)
            .arg("install")
            .spawn()
            .expect("Could not start process")
            .wait()
            .expect("Could not install dependencies");

        if !status.success() {
            panic!("npm install failed");
        }
        
    }

    automated("Creating new version of guides");
    // TODO don't just run the npm script here, do the work that it's doing in Rust
    if !dry_run {
        process::Command::new("npm")
            .current_dir(dir)
            .arg("run")
            .arg("release:guides:minor")
            .spawn()
            .expect("Could not start process")
            .wait()
            .expect("Failed to release guides.");
    }

    manual("Confirm new guides version is deployed before proceeding");
    manual("You are super duper sure it's deployed?");
}

/// This function runs the npm script in the project that
/// builds search index and then deploys.
pub fn publish_algolia(dir: &std::path::Path, dry_run: bool) {
    automated("Publishing Algolia index");

    if !dry_run {
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
