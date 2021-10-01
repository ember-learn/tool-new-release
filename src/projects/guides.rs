use crate::utils::prompt::{automated, manual};
use crate::utils::versions::CurrentVersions;
use serde::{Deserialize, Serialize};
use std::{path::Path, process};

#[derive(Serialize, Deserialize, Debug)]
pub struct VersionsYaml {
    #[serde(rename = "allVersions")]
    pub all_versions: Vec<String>,
    #[serde(rename = "currentVersion")]
    pub current_version: String,
    #[serde(rename = "ltsVersions")]
    pub lts_versions: Vec<String>,
}

pub fn run(dir: &Path, opts: &crate::Opts, versions: &CurrentVersions) {
    manual("Check for pending PRs: https://github.com/ember-learn/guides-source/pulls");

    automated("Cloning guide-source");
    let (guides_source_repo, guides_source_dir) =
        crate::clone::github(dir, "ember-learn", "guides-source");

    automated("Installing node dependencies");
    if !opts.dry_run {
        process::Command::new("node")
            .current_dir(&guides_source_dir)
            .arg("--version")
            .spawn()
            .expect("Could not start process")
            .wait()
            .expect("Could not install dependencies");
        process::Command::new("npm")
            .current_dir(&guides_source_dir)
            .arg("install")
            .spawn()
            .expect("Could not start process")
            .wait()
            .expect("Could not install dependencies");
    }

    automated("Updating versions list");
    let redeploy = update_versions_yml(&guides_source_dir, &versions.target);

    automated("Creating folder for previous release");
    create_release_version_folder(&guides_source_dir, versions, redeploy);

    automated("Updating links for previous release");
    let version_string = format!("guides/v{}", &versions.deployed);
    process::Command::new("npm")
        .current_dir(&guides_source_dir)
        .args(&[
            "run",
            "release:guides:links",
            &version_string,
            &versions.deployed.to_string(),
            "--silent",
        ])
        .spawn()
        .expect("Could not start scripts/update-version-links process")
        .wait()
        .expect("Could not update Guides links");

    println!("{:#?}", guides_source_dir);

    automated("Committing changes");
    process::Command::new("git")
        .current_dir(&guides_source_dir)
        .args(&[
            "switch",
            "-c",
            &format!(
                "automated-release-{}-{}",
                &versions.deployed.major, &versions.target.minor
            ),
        ])
        .spawn()
        .expect("Could not start scripts/update-version-links process")
        .wait()
        .expect("Could not update Guides links");

    let index = crate::git::add::add(&guides_source_repo);
    crate::git::commit::commit_with_message(index, &guides_source_repo, &version_string);
    process::Command::new("git").current_dir(&guides_source_dir).args(&["add", "-A"]).output().unwrap();
    process::Command::new("git").current_dir(&guides_source_dir).args(&["commit", "-m", &version_string]).output().unwrap();
    automated("Pushing changes");
    process::Command::new("git")
        .current_dir(&guides_source_dir)
        .args(&[
            "push",
            "-u",
            "origin",
            &format!(
                "master:automated-release-{}-{}",
                &versions.deployed.major, &versions.deployed.minor, 
            ),
            "--force"
        ])
        .spawn()
        .expect("Could not start scripts/update-version-links process")
        .wait()
        .expect("Could not update Guides links");

    // echo
    // echo "🤖 Committing changes and publishing branch to remote"
    // git add .
    // git commit -m "v$(echo $NEXT_VERSION)"
    // git push -u origin $TEMP_BRANCH
    // echo "  DONE"
    //
    // echo
    // echo "👩‍💻 Create pull request for $($TEMP_BRANCH): https://github.com/ember-learn/guides-source/compare/master...$TEMP_BRANCH"
    // read -n 1 -s -r -p "Press any key to continue"

    manual("Confirm new guides version is deployed before proceeding");
    manual("You are super duper sure it's deployed?");
    publish_algolia(opts, &guides_source_dir);
}

fn create_release_version_folder(
    guides_source_dir: &Path,
    versions: &CurrentVersions,
    redeploy: bool,
) {
    if redeploy {
        println!("Redeploying {}", versions.target.to_string());
        return;
    }

    let mut content_dir = guides_source_dir.to_path_buf();
    content_dir.push("guides/release");
    let mut target_dir = guides_source_dir.to_path_buf();
    target_dir.push("guides");
    target_dir.push(format!("v{}", versions.deployed.to_string()));
    std::fs::create_dir(&target_dir).unwrap();

    fs_extra::dir::copy(content_dir, target_dir, &fs_extra::dir::CopyOptions::new()).unwrap();
}

/// This function runs the npm script in the project that
/// builds search index and then deploys.
fn publish_algolia(opts: &crate::Opts, dir: &Path) {
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

fn update_versions_yml(guides_source_dir: &Path, version: &semver::Version) -> bool {
    let mut versions_file = guides_source_dir.to_path_buf();
    versions_file.push("guides/versions.yml");

    let data = std::fs::read_to_string(&versions_file).expect("Unable to read file");
    let mut res: VersionsYaml = serde_yaml::from_str(&data).expect("Unable to parse");

    let redeploy = res.current_version == version.to_string();
    if !redeploy {
        res.all_versions.push(format!("v{}", version.to_string()));
        res.current_version = format!("v{}", version.to_string());
        let yml = serde_yaml::to_string(&res).unwrap();

        std::fs::write(versions_file, yml).unwrap();
    }

    redeploy
}