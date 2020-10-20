use crate::utils::{pause, read_input};
use git2::Repository;
use process::ExitStatus;
use std::{path::PathBuf, process};

fn clone_repos(folder: &mut PathBuf) -> Result<Repository, git2::Error> {
    println!("🤖 Cloning guides-source");
    folder.push("guides-source");
    let repo = Repository::clone("https://github.com/ember-learn/guides-source.git", &folder)?;
    folder.pop();

    Ok(repo)
}

pub fn deploy_guides(mut dir: &mut PathBuf) -> Result<ExitStatus, std::io::Error> {
    println!("Beginning deploy for: Guides\n");
    println!("👩‍💻 Check for pending PRs: https://github.com/ember-learn/guides-source/pulls");
    pause();

    clone_repos(&mut dir).unwrap();

    println!("🤖 Installing node dependencies");
    dir.push("guides-source");
    process::Command::new("npm")
        .current_dir(&dir)
        .arg("install")
        .spawn()
        .expect("Could not install dependencies")
        .wait()?;

    println!("🤖 Creating new version of guides");
    process::Command::new("npm")
        .current_dir(&dir)
        .arg("run")
        .arg("release:guides:minor")
        .spawn()
        .expect("Failed to release guides.")
        .wait()?;

    println!("👩‍💻 Confirm new guides version is deployed before proceeding");
    pause();

    println!("👩‍💻 You are super duper sure it's deployed?");
    pause();

    println!("🤖 Publishing algolia index");
    process::Command::new("npm")
        .arg("run")
        .arg("release:search")
        .spawn()
        .expect("Failed to publish algolia index.")
        .wait()?;

    std::process::exit(0)
}
