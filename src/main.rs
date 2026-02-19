mod cli;
mod commands;
mod confirmation;
mod logger;
#[cfg(test)]
pub mod volumes;

use clap::Parser;
use cli::Cli;
use log::trace;
use owo_colors::OwoColorize;
use time::format_description;

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
    let mut cli = Cli::try_parse()?;
    cli.datetime_format = format_description::parse("[day]-[month]-[year] [hour]:[minute]")?;

    logger::init(&cli);

    trace!("Logger initialized.");
    trace!("{cli}");

    match cli.execute_command() {
        Ok(()) => trace!("Successfully executed command."),
        Err(err) => eprintln!("An error occurred during running command: {err}"),
    }

    Ok(())
}
