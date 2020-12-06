use structopt::{clap::arg_enum, StructOpt};
mod projects {
    pub mod api;
    pub mod guides;
}
mod utils;

arg_enum! {
    #[derive(Debug, StructOpt)]
    enum Step {
        Guides,
        Api,
    }
}

#[derive(Debug, StructOpt)]
struct Opts {
    #[structopt(short, long, possible_values = &Step::variants(), case_insensitive = true)]
    /// Pick which project to run the deploy pipeline for.
    step: Option<Step>,
}

fn main() {
    let mut dir = tempfile::tempdir().unwrap().into_path();

    let opts = Opts::from_args();

    match opts.step {
        Some(Step::Guides) => {
            crate::projects::guides::deploy(&mut dir).unwrap();
        }
        Some(Step::Api) => {
            crate::projects::api::deploy(&mut dir).unwrap();
        }
        None => {
            crate::projects::guides::deploy(&mut dir).unwrap();
            crate::projects::api::deploy(&mut dir).unwrap();
        }
    };

    println!("Finished!");
}
