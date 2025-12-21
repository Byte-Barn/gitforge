use anyhow::Result;
use clap::Subcommand;
use crate::commands::gitignore;
use crate::commands::issue;
use crate::commands::license;
use crate::commands::pr;

/// Subcommands for `gitcraft preview ...`
#[derive(Subcommand)]
pub enum Command {
    /// Preview a gitignore template
    Gitignore(gitignore::preview::PreviewArgs),
    /// Preview an issue template
    Issue(issue::preview::PreviewArgs),
    /// Preview a license
    License(license::preview::PreviewArgs),
    /// Preview a pull request template
    Pr(pr::preview::PreviewArgs),
}

impl Command {
    pub fn execute(&self) -> Result<()> {
        match self {
            Self::Gitignore(args) => gitignore::Command::Preview(args.clone()).execute(),
            Self::Issue(args) => issue::Command::Preview(args.clone()).execute(),
            Self::License(args) => license::Command::Preview(args.clone()).execute(),
            Self::Pr(args) => pr::Command::Preview(args.clone()).execute(),
        }
    }
}