/// Creates a commit in the repository.
pub fn commit(mut index: git2::Index, glitch_repo: &git2::Repository, version: &str) {
    let tree_id = index.write_tree().unwrap();
    let tree = glitch_repo.find_tree(tree_id).unwrap();
    let sig = glitch_repo.signature().unwrap();
    let head_id = glitch_repo.refname_to_id("HEAD").unwrap();
    let parent = glitch_repo.find_commit(head_id).unwrap();
    let _commit_id = glitch_repo
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
