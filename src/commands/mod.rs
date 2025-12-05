use clap::Subcommand;

pub mod base;
pub mod gitignore;
pub mod issue;
pub mod pr;
pub mod license;

pub mod add;
pub mod list;
pub mod preview;

#[derive(Subcommand)]
pub enum Command {
    #[command(subcommand)]
    /// Add issue templates (e.g. `gitcraft add issue-template`)
    Add(add::Command),

    #[command(subcommand)]
    /// List issue templates (e.g. `gitcraft list issue-templates`)
    List(list::Command),

    #[command(subcommand)]
    /// Preview an issue template (e.g. `gitcraft preview issue-template`)
    Preview(preview::Command),
}

impl Command {
    pub fn run(&self) -> anyhow::Result<()> {
        match self {
            Self::Add(cmd) => cmd.execute(),
            Self::List(cmd) => cmd.execute(),
            Self::Preview(cmd) => cmd.execute(),
        }
    }
}
