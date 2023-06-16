use std::path::{Path};
use crate::utils::prompt::{manual, pause};

use std::process::Command;

fn clone(root: &Path, src: String) -> git2::Repository {

    Command::new("git")
        .arg("clone")
        .arg(src)
        .arg(root)
        .spawn()
        .expect("Could not start git")
        .wait()
        .expect("could not clone repo");
    
    let repo = git2::Repository::open(&root).unwrap();
    repo
}


pub fn clone_or_skip(root: &Path, src: String) -> git2::Repository {
    if !root.read_dir().unwrap().next().is_none() {
        let repo = git2::Repository::open(&root).unwrap();
        manual("we're reusing the existing path. update it manually and continue the process");
        pause();
        repo
    } else {
        return clone(root, src);
    }
}
