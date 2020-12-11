use std::str::FromStr;

use structopt::StructOpt;
mod projects {
    pub mod api;
    pub mod guides;
}
mod utils;

#[derive(Debug, StructOpt)]
enum Step {
    Guides,
    Api,
}

impl FromStr for Step {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "guides" => Ok(Step::Guides),
            "api" => Ok(Step::Api),
            other => {
                println!("Step \"{}\" not recognized.", other);
                std::process::exit(1);
            }
        }
    }

    type Err = std::string::ParseError;
}

#[derive(Debug, StructOpt)]
struct Opts {
    #[structopt(short, long)]
    step: Option<Step>,
}

fn main() {
    let mut dir = tempfile::tempdir().unwrap().into_path();

    let opts = Opts::from_args();

    match opts.step {
        Some(Step::Guides) => {
            crate::projects::guides::deploy_guides(&mut dir).unwrap();
        }
        Some(Step::Api) => {
            crate::projects::api::deploy_api_documentation(&mut dir).unwrap();
        }
        None => {
            crate::projects::guides::deploy_guides(&mut dir).unwrap();
            crate::projects::api::deploy_api_documentation(&mut dir).unwrap();
        }
    };

    println!("Finished!");
}
