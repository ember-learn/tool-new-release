use git2::{Cred, RemoteCallbacks};
use std::{
    env,
    path::{Path, PathBuf},
};
use tempfile::tempdir_in;

pub fn clone(root: &Path, src: String) -> (git2::Repository, PathBuf) {
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
        Cred::ssh_key(
            username_from_url.unwrap(),
            None,
            Path::new(&format!("{}/.ssh/id_rsa", env::var("HOME").unwrap())),
            None,
        )
    });

    // Prepare fetch options.
    let mut fo = git2::FetchOptions::new();
    fo.remote_callbacks(callbacks);

    // Prepare builder.
    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fo);

    let dir = tempdir_in(root).unwrap().into_path();
    let repo = builder.clone(src.as_str(), &dir).unwrap();

    // let repo = git2::Repository::clone(src.as_str(), &dir).unwrap();
    (repo, dir)
}
