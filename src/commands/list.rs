use anyhow::Result;
use clap::Subcommand;

use crate::commands::gitignore;
use crate::commands::issue;
use crate::commands::license;
use crate::commands::pr;

/// Subcommands for `gitcraft list ...`
#[derive(Subcommand)]
pub enum Command {
    /// List available gitignore templates
    #[command(alias = "gitignores")]
    Gitignore(gitignore::list::ListArgs),
    /// List available issue templates
    #[command(alias = "issues")]
    Issue(issue::list::ListArgs),
    /// List available licenses
    #[command(alias = "licenses")]
    License(license::list::ListArgs),
    /// List available pull request templates
    #[command(alias = "prs")]
    Pr(pr::list::ListArgs),
}

impl Command {
    pub fn execute(&self) -> Result<()> {
        match self {
            Self::Gitignore(args) => gitignore::Command::List(args.clone()).execute(),
            Self::Issue(args) => issue::Command::List(args.clone()).execute(),
            Self::License(args) => license::Command::List(args.clone()).execute(),
            Self::Pr(args) => pr::Command::List(args.clone()).execute(),
        }
    }
}