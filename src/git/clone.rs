use std::path::{Path, PathBuf};
use tempfile::tempdir_in;

pub fn clone(root: &Path, src: String) -> (git2::Repository, PathBuf) {
    let dir = tempdir_in(root).unwrap().into_path();
    let repo = git2::Repository::clone(src.as_str(), &dir).unwrap();
    (repo, dir)
}
