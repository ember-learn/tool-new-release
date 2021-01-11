use structopt::{clap::arg_enum, StructOpt};
use utils::prompt;
mod projects {
    pub mod api;
    pub mod blog_post;
    pub mod glitch;
    pub mod guides;
    pub mod release_pages;
    pub mod wikipedia;
}
mod repo;
mod utils;

arg_enum! {
    #[derive(Debug, StructOpt)]
    enum Project {
        Guides,
        Api,
        BlogPost,
        ReleasePages,
        Glitch,
        Wikipedia
    }
}

#[derive(Debug, StructOpt)]
pub struct Opts {
    #[structopt(short, long, possible_values = &Project::variants(), case_insensitive = true)]
    /// Pick which project to run the deploy pipeline for.
    project: Option<Project>,

    /// Run the deploy pipeline without actually deploying.
    /// Useful for understanding all the necessary steps, or when working on the pipeline itself.
    #[structopt(long)]
    dry_run: bool,

    #[structopt(short, long)]
    version: String,
}

fn main() {
    let mut dir = tempfile::tempdir().unwrap().into_path();
    let opts = Opts::from_args();

    println!(
        "Ember Core Learning team release process.

You will be presented with instructions.
There will be some interactive and manual steps, so please read the instructions carefully.

Legend:
* {} - User input will be required for this task
* {} - This step is automated
",
        utils::TaskType::Manual,
        utils::TaskType::Automated
    );
    prompt(utils::TaskType::Manual, "Ready to start?");

    match opts.project {
        Some(Project::Guides) => {
            println!("Pipelines:\n · Guides");
            crate::projects::guides::run(&mut dir, &opts);
            println!("Pipelines:\n ✓ Guides");
        }
        Some(Project::Api) => {
            println!("Pipelines:\n · API");
            crate::projects::api::run(&mut dir, &opts);
            println!("Pipelines:\n ✓ API");
        }
        Some(Project::BlogPost) => {
            println!("Pipelines:\n · Blog post");
            crate::projects::blog_post::run();
            println!("Pipelines:\n ✓ Blog post");
        }
        Some(Project::ReleasePages) => {
            println!("Pipelines:\n · Release pages\n");
            crate::projects::release_pages::run();
            println!("Pipelines:\n ✓ Release pages");
        }
        Some(Project::Glitch) => {
            println!("Pipelines:\n · Glitch\n");
            crate::projects::glitch::run(&mut dir, &opts);
            println!("Pipelines:\n ✓ Glitch");
        }
        Some(Project::Wikipedia) => {
            println!("Pipelines:\n · Wikipedia\n");
            crate::projects::wikipedia::run();
            println!("Pipelines:\n ✓ Wikipedia");
        }
        None => {
            println!("Pipelines:\n · Guides\n · API\n · Blog post\n · Release pages\n · Glitch\n · Wikipedia");
            crate::projects::guides::run(&mut dir, &opts);
            println!("Pipelines:\n ✓ Guides\n · API\n · Blog post\n · Release pages\n · Glitch\n · Wikipedia");
            crate::projects::api::run(&mut dir, &opts);
            println!("Pipelines:\n ✓ Guides\n ✓ API\n · Blog post\n · Release pages\n · Glitch\n · Wikipedia");
            crate::projects::blog_post::run();
            println!("Pipelines:\n ✓ Guides\n ✓ API\n ✓ Blog post\n · Release pages\n · Glitch\n · Wikipedia");
            crate::projects::release_pages::run();
            println!("Pipelines:\n ✓ Guides\n ✓ API\n ✓ Blog post\n ✓ Release pages\n · Glitch\n · Wikipedia");
            crate::projects::glitch::run(&mut dir, &opts);
            println!("Pipelines:\n ✓ Guides\n ✓ API\n ✓ Blog post\n ✓ Release pages\n ✓ Glitch\n · Wikipedia");
            crate::projects::wikipedia::run();
            println!("Pipelines:\n ✓ Guides\n ✓ API\n ✓ Blog post\n ✓ Release pages\n ✓ Glitch\n ✓ Wikipedia");
        }
    };
}
