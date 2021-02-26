use cli::Opts;
use std::path::Path;
use structopt::StructOpt;

mod projects {
    pub mod api;
    pub mod blog_post;
    pub mod bot;
    pub mod glitch;
    pub mod guides;
    pub mod release_pages;
    pub mod wikipedia;
}
mod cli;
mod clone;
mod pipeline;
mod utils;

fn main() {
    let temp = tempfile::tempdir().unwrap();
    let dir: &Path = temp.path();
    let opts = cli::Opts::from_args();

    crate::cli::intro();
    let versions = crate::cli::ask_version();
    let mut pipeline = pipeline::Pipeline::new();
    let chosen_project_indices = crate::cli::ask_projects(&pipeline);
    pipeline.run(chosen_project_indices, &dir, &opts, &versions);

    temp.close().unwrap();
}


