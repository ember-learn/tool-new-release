use git2::{Direction, PushOptions, Repository};

#[allow(dead_code)]
pub fn push_to_git(repo: &Repository) -> std::result::Result<(), git2::Error> {
    let mut cb = git2::RemoteCallbacks::new();
    cb.push_update_reference(|refname, status| {
        println!("-> {}", refname);
        if status.is_some() {
            panic!("Could not push to remote");
        } else {
            Ok(())
        }
    });
    cb.credentials(|_url, username_from_url, _allowed_types| {
        println!("CRED->{:?}", _allowed_types);
        git2::Cred::username(username_from_url.unwrap())
    });
    cb.certificate_check(|_cert, str| {
        println!("CERT {:?}", str);
        true
    });

    let mut opts = PushOptions::new();
    opts.remote_callbacks(cb);

    let mut glitch = repo.find_remote("origin").unwrap();
    glitch.connect(Direction::Push).unwrap();
    glitch.push(&["refs/heads/master"] as &[&str], Some(&mut opts))
}
