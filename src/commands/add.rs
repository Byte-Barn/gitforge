use anyhow::Result;
use clap::Subcommand;
use crate::commands::{gitignore, issue, license, pr};

/// Subcommands for `gitforge add ...`
#[derive(Subcommand)]
pub enum Command {
    /// Add gitignore templates
    Gitignore(gitignore::add::AddArgs),

    /// Add an issue template
    #[command(name = "issue-template", alias = "issue")]
    IssueTemplate(issue::add::AddArgs),

    /// Add a pull request template
    #[command(name = "pr-template", alias = "pr")]
    PrTemplate(pr::add::AddArgs),

    /// Add a license
    License(license::add::AddArgs),
}

impl Command {
    pub fn execute(&self) -> Result<()> {
        match self {
            Self::Gitignore(args) => gitignore::Command::Add((*args).clone()).execute(),
            Self::IssueTemplate(args) => issue::Command::Add((*args).clone()).execute(),
            Self::PrTemplate(args) => pr::Command::Add((*args).clone()).execute(),
            Self::License(args) => license::Command::Add((*args).clone()).execute(),
        }
    }
}