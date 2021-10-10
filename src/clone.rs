use std::path::{Path, PathBuf};

pub fn github(
    root: &std::path::Path,
    organization: &str,
    project: &str,
) -> (git2::Repository, PathBuf) {
    let src = format!("https://github.com/{}/{}.git", organization, project);

    crate::git::clone::clone(root, src)
}

pub fn glitch(root: &Path, src: &str) -> (git2::Repository, PathBuf) {
    crate::git::clone::clone(root, src.to_string())
}