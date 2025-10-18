mod cli;
mod commands;
mod logger;

use clap::Parser;
use cli::Cli;
use log::trace;
use owo_colors::OwoColorize;

fn main() {
    match run() {
        Ok(()) => (),
        Err(err) => println!(
            "{}.\n {err}",
            "An error occurred during running application".bright_red()
        ),
    };
}

fn run() -> anyhow::Result<()> {
    let cli = Cli::try_parse().map_err(|err| anyhow::anyhow!(err))?;

    logger::init(&cli);

    trace!("Logger initialized.");
    trace!("{cli}");

    match cli.execute_command() {
        Ok(()) => trace!("Successfully executed command."),
        Err(err) => eprintln!("An error occurred during running command: {err}"),
    }

    Ok(())
}
