use std::path::PathBuf;

pub fn push(dir: PathBuf) {
    std::process::Command::new("git")
        .current_dir(&dir)
        .args(&["push"])
        .spawn()
        .unwrap()
        .wait()
        .expect("git status");
}
