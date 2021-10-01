/// Creates a commit in the repository.
pub fn commit(mut index: git2::Index, repo: &git2::Repository, version: &str) {
    let tree_id = index.write_tree().unwrap();
    let tree = repo.find_tree(tree_id).unwrap();
    let sig = repo.signature().unwrap();
    let head_id = repo.refname_to_id("HEAD").unwrap();
    let parent = repo.find_commit(head_id).unwrap();
    let _commit_id = repo
        .commit(
            Some("HEAD"),
            &sig,
            &sig,
            &format!("{} (glitch)", version),
            &tree,
            &[&parent],
        )
        .unwrap();
}

pub fn commit_with_message(mut index: git2::Index, repo: &git2::Repository, message: &str) {
    let tree_id = index.write_tree().unwrap();
    let tree = repo.find_tree(tree_id).unwrap();
    let sig = repo.signature().unwrap();
    let head_id = repo.refname_to_id("HEAD").unwrap();
    let parent = repo.find_commit(head_id).unwrap();
    let _commit_id = repo
        .commit(
            Some("HEAD"),
            &sig,
            &sig,
            message,
            &tree,
            &[&parent],
        )
        .unwrap();
}
