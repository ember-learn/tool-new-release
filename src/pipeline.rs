use std::{fmt::Display, path::Path};

use crate::{
    cli::{Opts, Project},
    utils,
};

#[derive(Clone)]
pub enum PipelineStatus {
    Skip,
    Queued,
    InProgress,
    Finished,
}

impl std::fmt::Display for PipelineStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PipelineStatus::Skip => Ok(()),
            PipelineStatus::Queued => write!(f, "·"),
            PipelineStatus::InProgress => write!(f, "♲"),
            PipelineStatus::Finished => write!(f, "✓"),
        }
    }
}

#[derive(Clone)]
pub struct PipelineProject {
    pub project: crate::cli::Project,
    pub status: PipelineStatus,
}

impl PipelineProject {
    pub fn new(project: Project) -> Self {
        Self {
            project,
            status: PipelineStatus::Skip,
        }
    }

    pub fn run(&self, dir: &Path, opts: &Opts, versions: &utils::versions::CurrentVersions) {
        match self.project {
            crate::cli::Project::Guides => {
                crate::projects::guides::run(dir, opts);
            }
            crate::cli::Project::Api => {
                crate::projects::api::run(dir, opts);
            }
            crate::cli::Project::BlogPost => {
                crate::projects::blog_post::run();
            }
            crate::cli::Project::ReleasePages => {
                crate::projects::release_pages::run();
            }
            crate::cli::Project::Glitch => {
                crate::projects::glitch::run(dir, opts, &versions.deployed);
            }
            crate::cli::Project::Wikipedia => {
                crate::projects::wikipedia::run();
            }
            crate::cli::Project::Bot => {
                crate::projects::bot::run(&versions.deployed);
            }
        };
    }

    pub fn queued(&mut self) {
        self.status = PipelineStatus::Queued;
    }

    pub fn in_progress(&mut self) {
        self.status = PipelineStatus::InProgress;
    }

    pub fn finished(&mut self) {
        self.status = PipelineStatus::Finished;
    }
}

impl std::fmt::Display for PipelineProject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, " {} {}", self.status, self.project)
    }
}

pub struct Pipeline(pub Vec<PipelineProject>);

impl Pipeline {
    pub fn new() -> Pipeline {
        Pipeline(vec![
            PipelineProject::new(Project::Guides),
            PipelineProject::new(Project::Api),
            PipelineProject::new(Project::BlogPost),
            PipelineProject::new(Project::ReleasePages),
            PipelineProject::new(Project::Glitch),
            PipelineProject::new(Project::Wikipedia),
            PipelineProject::new(Project::Bot),
        ])
    }

    pub fn run(
        &mut self,
        chosen_project_indices: Vec<usize>,
        dir: &Path,
        opts: &Opts,
        versions: &crate::utils::versions::CurrentVersions,
    ) {
        for index in chosen_project_indices {
            self.0[index].queued();
        }

        for i in 0..self.0.len() {
            match self.0.get_mut(i).unwrap().status {
                crate::pipeline::PipelineStatus::Skip => {}
                _ => {
                    self.0.get_mut(i).unwrap().in_progress();
                    println!("➡{}\n", self);

                    let project = self.0.get_mut(i).unwrap();
                    project.run(dir, opts, versions);
                    project.finished();
                }
            }
        }

        // for mut project in self.0 {
        //     match project.status {
        //         crate::pipeline::PipelineStatus::Skip => {}
        //         _ => {
        //             project.in_progress();
        //             println!("➡{}\n", self);
        //             project.run(&dir, &opts, &versions);
        //             project.finished();
        //         }
        //     }
        // }
    }
}

impl Display for Pipeline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for project in &self.0 {
            match project.status {
                PipelineStatus::Skip => {}
                _ => {
                    write!(f, "{}", project)?;
                }
            };
        }

        Ok(())
    }
}

impl IntoIterator for Pipeline {
    type Item = PipelineProject;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

// struct PipelineIntoIterator {
//     pipeline: Pipeline,
//     index: usize,
// }

// impl Iterator for PipelineIntoIterator {
//     type Item = PipelineProject;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.index >= self.pipeline.0.len() {
//             return None;
//         }
//         self.index += 1;

//         let project = self.pipeline.0[self.index];
//         match project.status {
//             PipelineStatus::Skip => self.next(),
//             _ => Some(project),
//         }
//     }
// }
