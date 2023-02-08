use structopt::{clap::arg_enum, StructOpt};

arg_enum! {
    #[derive(Debug, StructOpt, Clone)]
    pub enum Project {
        GuidesContent,
        GuidesSearch,
        Api,
        BlogPost,
        ReleasePages,
        Glitch,
        Wikipedia,
        Bot
    }
}

/// Ember Learning team release helper.
#[derive(Debug, StructOpt)]
pub struct Opts {
    #[structopt(short, long, possible_values = &Project::variants(), case_insensitive = true)]
    /// Pick which project to run the deploy pipeline for.
    pub project: Option<Project>,

    /// Run the deploy pipeline without actually deploying.
    /// Useful for understanding all the necessary steps, or when working on the pipeline itself.
    #[structopt(long)]
    pub dry_run: bool,

    /// Toggles
    #[structopt(long)]
    pub major_version: bool,
}

pub fn ask_projects(pipeline: &crate::pipeline::Pipeline) -> Vec<usize> {
    let mut project_selector = dialoguer::MultiSelect::new();
    for option in pipeline.0.iter() {
        project_selector.item_checked(option, true);
    }
    let chosen_project_indices = project_selector
        .with_prompt("Choose projects to release")
        .interact()
        .unwrap();
    chosen_project_indices
}

pub fn ask_version(major_version: bool) -> crate::utils::versions::CurrentVersions {
    let versions = crate::utils::versions::CurrentVersions::from_guides(major_version);
    let chosen = dialoguer::Select::new()
        .default(1)
        .with_prompt("Pick version you wish to release")
        .items(&[&versions.deployed, &versions.target])
        .interact()
        .unwrap();

    match chosen {
        0 => crate::utils::versions::CurrentVersions::from_target_version(&versions.deployed),
        1 => crate::utils::versions::CurrentVersions::from_versions(&versions),
        _ => unreachable!(),
    }
}

pub fn intro() {
    println!(
        "Ember Core Learning team release process.

You will be presented with instructions.
There will be some interactive and manual steps, so please read the instructions carefully.

Legend:
* {} - User input will be required for this task
* {} - This step is automated
",
        crate::utils::prompt::TaskType::Manual,
        crate::utils::prompt::TaskType::Automated
    );
}
