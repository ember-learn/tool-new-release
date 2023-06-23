use std::path::{PathBuf};
use crate::utils::prompt::{automated, yes_no};
use std::fs;
use dirs;

mod projects {
    pub mod api;
    pub mod blog_post;
    pub mod bot;
    pub mod glitch;
    pub mod guides;
    pub mod wikipedia;
}
mod cli;
mod args;

use args::ReleaseArgs;
use clap::Parser;

pub mod utils;
mod git {
    pub mod add;
    pub mod clone;
    pub mod commit;
    pub mod push;
}
mod glitch {
    pub mod push;
}

fn get_project_name(project: &args::Project) -> &'static str {
    match project {
        args::Project::Guides => "Guides",
        args::Project::GuidesSearch => "Guides",
        args::Project::ApiDocs => "ApiDocs",
        args::Project::BlogPost => "BlogPost",
        args::Project::Glitch { version: _ } => "Glitch",
        args::Project::Wikipedia => "Wikipedia",
        args::Project::Bot { major_version: _ } => "Bot",
    }
}

fn main() {
    let args = ReleaseArgs::parse();

    let mut dir = PathBuf::new();

    dir.push(dirs::home_dir().expect("couldn't find home dir"));
    dir.push("tool-new-release-working-dir");
    dir.push(get_project_name(&args.project));

    let dir = dir.as_path();
    
    let local_dir = format!("~/tool-new-release-working-dir/{}", get_project_name(&args.project));

    automated(format!("starting the process in the local dir {} now", local_dir).as_str());

    if dir.exists() {
        if yes_no(format!("Working directory {} already exists, do you want to delete it? y/n", local_dir).as_str()) {
            println!("Deleting now!");
            fs::remove_dir_all(dir).expect("could not delete working directory");
            fs::create_dir_all(dir).expect("Could not create working directory");
        }
    } else {
        fs::create_dir_all(dir).expect("Could not create working directory");
    }

    match args.project {
        args::Project::Guides => projects::guides::run(dir, args.dry_run),
        args::Project::GuidesSearch => projects::guides::publish_algolia(dir, args.dry_run),
        args::Project::ApiDocs => projects::api::run(dir, args.dry_run),
        args::Project::BlogPost => projects::blog_post::run(),
        args::Project::Glitch { version } => projects::glitch::run(dir, args.dry_run, version),
        args::Project::Wikipedia => projects::wikipedia::run(),
        args::Project::Bot { major_version } => {
            let versions = crate::cli::ask_version(major_version);
            projects::bot::run(&versions.target)
        },
    };
}
