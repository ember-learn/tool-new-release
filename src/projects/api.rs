use crate::utils::prompt::automated;
use crate::Opts;
use std::process;

pub fn run(dir: &std::path::Path, opts: &Opts) {
    let vars = crate::utils::op::get_api_docs_vars();
    let (_, jsonapi_docs_dir) = crate::clone::github(dir, "ember-learn", "ember-jsonapi-docs");

    automated("Installing node dependencies");
    if !opts.dry_run {
        process::Command::new("yarn")
            .current_dir(&jsonapi_docs_dir)
            .arg("install")
            .spawn()
            .expect("Could not spawn new process")
            .wait()
            .expect("Could not install dependencies");
    }

    automated("Generating API documentation…");
    if !opts.dry_run {
        process::Command::new("yarn")
            .current_dir(&jsonapi_docs_dir)
            .envs(vars)
            .args(&["run", "start", "--sync"])
            .spawn()
            .unwrap()
            .wait()
            .expect("Could not compile API documentation");
    }
}
