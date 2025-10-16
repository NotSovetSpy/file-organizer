use log::LevelFilter;

use crate::cli::Cli;

pub fn init(cli: &Cli) {
    let mut logger = env_logger::builder();

    // Order is important, to apply lowest log level.
    if cli.trace {
        logger.filter_level(LevelFilter::Trace);
    } else if cli.verbose {
        logger.filter_level(LevelFilter::Debug);
    } else {
        logger.filter_level(LevelFilter::Info);
    }

    logger.init();
}
