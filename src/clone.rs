use crate::utils::prompt::automated;
use std::path::PathBuf;

pub fn github(
    root: &std::path::Path,
    organization: &str,
    project: &str,
) -> (git2::Repository, PathBuf) {
    let src = format!("https://git@github.com/{}/{}.git", organization, project);
    automated(format!("Cloning {}", src).as_str());

    crate::git::clone::clone(root, src)
}
