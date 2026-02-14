use std::fmt::Display;

use clap::Subcommand;
use owo_colors::OwoColorize;

use crate::{
    cli::Cli,
    commands::{find::FindCommand, sort::SortCommand},
};

mod find;
mod sort;

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(name = "find", about = "Find files with specific criteria")]
    Find(FindCommand),
    #[command(name = "sort", about = "Sort")]
    Sort(SortCommand),
}

impl Commands {
    pub fn execute(&self, context: &Cli) -> anyhow::Result<()> {
        match self {
            Commands::Find(cmd) => cmd.execute(context),
            Commands::Sort(cmd) => cmd.execute(context),
        }
    }
}

impl Display for Commands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Commands::Find(cmd) => {
                writeln!(f, "{}: {}", "command_name".bright_cyan(), "find")?;
                writeln!(f, "{}", cmd)?;
                Ok(())
            }
            Commands::Sort(_cmd) => {
                // TODO: implemet
                unimplemented!()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::panic::catch_unwind;

    use crate::cli::Cli;
    use clap::Parser;

    #[test]
    fn test_find_configuration_syntax() {
        let args = vec!["fo", "find", "/tmp"];
        let is_panic_result = catch_unwind(|| Cli::try_parse_from(args));
        assert!(is_panic_result.is_ok());
    }
}
