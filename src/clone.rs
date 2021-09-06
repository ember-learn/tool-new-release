use std::path::{Path, PathBuf};
use tempfile::tempdir_in;

pub fn github(
    root: &std::path::Path,
    organization: &str,
    project: &str,
) -> (git2::Repository, PathBuf) {
    let src = format!("https://github.com/{}/{}.git", organization, project);

    let dir = tempdir_in(root).unwrap().into_path();
    let repo = git2::Repository::clone(src.as_str(), &dir).unwrap();

    (repo, dir)
}

pub fn glitch(root: &Path, src: &str) -> (git2::Repository, PathBuf) {
    let dir = tempdir_in(root).unwrap().into_path();
    let repo = git2::Repository::clone(src, &dir).unwrap();

    (repo, dir)
}
