use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct ReleaseArgs {
    #[clap(subcommand)]
    pub project: Project,

    /// Run the deploy pipeline without actually deploying.
    /// Useful for understanding all the necessary steps, or when working on the pipeline itself.
    #[arg(long, global = true)]
    pub dry_run: bool,
}

#[derive(Debug, Subcommand)]
pub enum Project {
    /// Release the Guides guides.emberjs.com
    Guides,

    /// Run algolia on the guides
    GuidesSearch,

    /// Release the API Docs api.emberjs.com
    ApiDocs,

    /// Prepare the release blog post on blog.emberjs.com
    BlogPost,

    /// Update details on wikipedia
    Wikipedia,

    /// Update the reminder bot
    Bot {
        /// Set this to true if you're doing a major version release
        #[arg(long, global = true)]
        major_version: bool,
    },
}
