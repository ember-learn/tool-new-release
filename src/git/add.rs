/// Stages all changed files in the repository.
pub fn add(glitch_repo: &git2::Repository) -> git2::Index {
    let mut index = glitch_repo.index().unwrap();
    index
        .add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)
        .unwrap();
    index.write().unwrap();
    index
}
