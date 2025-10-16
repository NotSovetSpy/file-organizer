use clap::Subcommand;

use crate::cli::Cli;

#[derive(Subcommand, Debug)]
pub enum Commands {}

impl Commands {
    pub fn execute(&self, _context: &Cli) -> anyhow::Result<()> {
        Ok(())
    }
}
