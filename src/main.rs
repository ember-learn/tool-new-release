use structopt::{clap::arg_enum, StructOpt};
mod projects {
    pub mod api;
    pub mod blog_post;
    pub mod guides;
}
mod repo;
mod utils;

arg_enum! {
    #[derive(Debug, StructOpt)]
    enum Project {
        Guides,
        Api,
        BlogPost
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
}

fn main() {
    let mut dir = tempfile::tempdir().unwrap().into_path();

    let opts = Opts::from_args();

    match opts.project {
        Some(Project::Guides) => {
            println!("Pipelines:\n · Guides");
            crate::projects::guides::deploy(&mut dir, &opts);
            println!("Pipelines:\n ✓ Guides");
        }
        Some(Project::Api) => {
            println!("Pipelines:\n · API");
            crate::projects::api::deploy(&mut dir, &opts);
            println!("Pipelines:\n ✓ API");
        }
        Some(Project::BlogPost) => {
            println!("Pipelines:\n · Blog post");
            crate::projects::blog_post::deploy();
            println!("Pipelines:\n ✓ Blog post");
        }
        None => {
            println!("Pipelines:\n · Guides\n · API\n · Blog post");
            crate::projects::guides::deploy(&mut dir, &opts);
            println!("Pipelines:\n ✓ Guides\n · API\n · Blog post");
            crate::projects::api::deploy(&mut dir, &opts);
            println!("Pipelines:\n ✓ Guides\n ✓ API\n · Blog post");
            crate::projects::blog_post::deploy();
            println!("Pipelines:\n ✓ Guides\n ✓ API\n ✓ Blog post");
        }
    };

    println!("Finished!");
}
