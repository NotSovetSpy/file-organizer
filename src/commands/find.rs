use std::{fmt::Display, path::PathBuf};

use clap::Parser;
use log::{debug, trace};
use owo_colors::OwoColorize;

use crate::cli::Cli;

mod file_list;
mod filtering;
mod printer;

pub(super) use file_list::FilesList;
pub(super) use filtering::*;
pub(super) use printer::*;

#[derive(Parser, Debug, Default)]
pub struct FindCommand {
    #[arg(help = "Directory to search", default_value = ".", requires = "filter")]
    directory: PathBuf,
    #[arg(
        short = 'n',
        long,
        name = "name",
        group = "filter",
        help = "Filter by file name"
    )]
    file_name: Option<String>,
    #[arg(short, long, group = "filter", help = "Filter by file size in bytes")]
    size: Option<String>,
    #[arg(short, long, group = "filter", help = "Filter by file extension")]
    ext: Option<String>,
    #[arg(
        short,
        long,
        group = "filter",
        help = "Filter by file creation datetime in format 'YYYY-MM-DD HH:MM'"
    )]
    datetime: Option<String>,
    #[arg(
        short,
        long,
        group = "filter",
        help = "Filter by file modification datetime in format 'YYYY-MM-DD HH:MM'"
    )]
    modified: Option<String>,
    #[arg(
        long = "regex",
        help = "Combine this with other filters to use regex for filtering"
    )]
    is_regex: bool,
    #[arg(
        short = 'a',
        long = "all",
        help = "Include hidden files (dotfiles) in the search"
    )]
    search_hidden: bool,
    #[arg(
        short = 'r',
        long = "recursive",
        help = "Search directories recursively"
    )]
    search_recursive: bool,
}

impl FindCommand {
    pub fn execute(&self, context: &Cli) -> anyhow::Result<()> {
        debug!("Executing 'find' command");
        trace!("with configuration: {self}");
        let files = FilesList::new(&self.directory, self.search_recursive, self.search_hidden)?;

        debug!("Filtering files based on provided criteria");
        let file_matcher = create_matcher_from_config(self, context)?;
        let mut matched_files = Vec::new();
        let mut total_files = 0;
        let mut total_matched_files = 0;
        for file in files {
            let file = file?;
            total_files += 1;
            if file_matcher.matches(&file, context)? {
                matched_files.push(file);
                total_matched_files += 1;
            }
        }

        print_files(context, &matched_files, total_files, total_matched_files)
    }
}

impl Display for FindCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}: {}",
            "filename".bright_cyan(),
            self.file_name.as_ref().unwrap_or(&String::from("None"))
        )?;
        writeln!(
            f,
            "{}: {}",
            "directory".bright_cyan(),
            self.directory.to_string_lossy()
        )?;
        writeln!(
            f,
            "{}: {}",
            "size".bright_cyan(),
            self.size
                .as_ref()
                .map(|s| s.to_string())
                .unwrap_or(String::from("None"))
        )?;
        writeln!(
            f,
            "{}: {}",
            "ext".bright_cyan(),
            self.ext.as_ref().unwrap_or(&String::from("None"))
        )?;
        writeln!(
            f,
            "{}: {}",
            "datetime".bright_cyan(),
            self.datetime.as_ref().unwrap_or(&String::from("None"))
        )?;
        writeln!(
            f,
            "{}: {}",
            "modified".bright_cyan(),
            self.modified.as_ref().unwrap_or(&String::from("None"))
        )?;
        writeln!(f, "{}: {}", "regex".bright_cyan(), self.is_regex)?;
        writeln!(
            f,
            "{}: {}",
            "search_hidden".bright_cyan(),
            self.search_hidden
        )?;
        writeln!(
            f,
            "{}: {}",
            "recursive".bright_cyan(),
            self.search_recursive
        )?;

        Ok(())
    }
}
