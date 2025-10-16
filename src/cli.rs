use clap::{Parser};
use log::trace;
use std::fmt::Display;
use owo_colors::OwoColorize;

use crate::commands::Commands;

#[derive(Parser, Debug)]
#[command(
    version,
    about = "Simple CLI file organizer", 
    long_about = None, 
    next_line_help = true
)]
pub struct Cli {
    #[arg(short, long)]
    pub verbose: bool,
    #[arg(long)]
    pub trace: bool,
    #[arg(short, long, global = true)]
    pub dry_run: bool,
    #[command(subcommand)]
    pub command: Option<Commands>
}

impl Display for Cli {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", "Parsed arguments:".bold())?;
        writeln!(f, "{}: {}", "verbose".bright_cyan(), self.verbose)?;
        writeln!(f, "{}: {}", "dry-run".bright_cyan(), self.dry_run)?;

        Ok(())
    }
}

impl Cli {
    pub fn execute_command(&self) -> anyhow::Result<()> {
        
        match &self.command {
            None => {
                trace!("No command provided to execute");
                Ok(())
            },
            Some(command) => {
                trace!("Found some command, executing...");
                command.execute(self)
            }
        }
    }
}