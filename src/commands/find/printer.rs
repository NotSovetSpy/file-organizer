use log::debug;
use owo_colors::OwoColorize;
use std::fs::DirEntry;
use time::OffsetDateTime;

use crate::cli::Cli;

pub fn print_files(
    context: &Cli,
    files: &[DirEntry],
    total_files: i32,
    total_matched_files: i32,
) -> anyhow::Result<()> {
    debug!("Printing matched files");
    if total_matched_files != 0 {
        println!("{}", "Matched files:".bold().bright_green());

        for file in files {
            let metadata = file.metadata()?;
            let file_size = metadata.len();
            let created_time = OffsetDateTime::from(metadata.created()?);

            let path = file.path().to_string_lossy().to_string();
            let size_str = file_size.to_string();
            let date_str = created_time.format(&context.datetime_format)?.to_string();

            println!(
                "{:<path_width$} {:>size_width$} {} {}",
                path.bright_cyan(),
                size_str.bright_yellow(),
                "bytes".bright_cyan(),
                date_str.bright_purple(),
                path_width = 100,
                size_width = 10
            );
        }
    }

    println!();
    println!(
        "{} {}",
        "Total files scanned:".bright_green(),
        total_files.to_string().bright_purple()
    );
    println!(
        "{} {}",
        "Total matched files:".bright_green(),
        total_matched_files.to_string().bright_purple()
    );

    Ok(())
}
