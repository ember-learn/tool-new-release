use crate::utils::prompt::automated;

pub fn github(
    root: &std::path::Path,
    organization: &str,
    project: &str,
) -> git2::Repository {
    let src = format!("git@github.com:{}/{}.git", organization, project);
    automated(format!("Cloning {} into tempdir {:?}", src, root).as_str());

    crate::git::clone::clone_or_skip(root, src)
}
