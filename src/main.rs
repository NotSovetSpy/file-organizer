mod cli;
mod commands;
mod logger;

use clap::Parser;
use cli::Cli;
use log::trace;

fn main() {
    match run() {
        Ok(()) => (),
        Err(help_message) => println!("{help_message}"),
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
