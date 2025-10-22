use clap::Parser;
use log::trace;
use owo_colors::OwoColorize;
use std::fmt::Display;
use time::format_description;

use crate::commands::Commands;

#[derive(Parser, Debug, Default)]
#[command(version, about = "Simple CLI file organizer", next_line_help = true)]
pub struct Cli {
    #[arg(short, long, global = true)]
    pub verbose: bool,
    #[arg(long, global = true)]
    pub trace: bool,
    #[arg(long, global = true)]
    pub dry_run: bool,
    #[command(subcommand)]
    pub command: Option<Commands>,
    #[clap(skip)]
    pub datetime_format: Vec<format_description::BorrowedFormatItem<'static>>,
}

impl Display for Cli {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", "Parsed arguments:".bold())?;
        writeln!(f, "{}: {}", "verbose".bright_cyan(), self.verbose)?;
        writeln!(f, "{}: {}", "trace".bright_cyan(), self.trace)?;
        writeln!(f, "{}: {}", "dry-run".bright_cyan(), self.dry_run)?;

        if let Some(command) = &self.command {
            writeln!(
                f,
                "{}:\n{}",
                "command".bright_cyan(),
                Cli::format_commands(command)
            )?;
        } else {
            writeln!(f, "{}: {}", "command".bright_cyan(), "None")?;
        }

        Ok(())
    }
}

impl Cli {
    pub fn execute_command(&self) -> anyhow::Result<()> {
        match &self.command {
            None => {
                trace!("No command provided to execute");
                Ok(())
            }
            Some(command) => {
                trace!("Found some command, executing...");
                command.execute(self)
            }
        }
    }

    fn format_commands(command: &Commands) -> String {
        let str_command = format!("{}", command);
        str_command
            .lines()
            .map(|line| format!("\t{}", line))
            .collect::<Vec<String>>()
            .join("\n")
    }
}
