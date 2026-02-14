use std::{
    fmt::{Debug, Display},
    path::PathBuf,
};

use clap::Parser;
use log::{debug, trace};

use crate::{cli::Cli, commands::sort::file_action::FileAction};
use owo_colors::OwoColorize;
use sort_directory::sort_directory;
// use transfer_files::transfer_files;

mod file_action;
mod sort_by;
mod sort_directory;
mod sorters;
mod transfer_files;

pub(super) use super::find::FilesList;
pub(super) use sort_by::SortBy;
pub(super) use sorters::*;

#[derive(Parser, Debug, Default)]
#[group(required = true, id = "filter", id = "action")]
pub struct SortCommand {
    #[arg(help = "Directory to search", default_value = ".")]
    directory: PathBuf,
    #[arg(
        short,
        long,
        help = "Copy files instead of moving them",
        group = "action"
    )]
    copy: bool,
    #[arg(
        short,
        long = "move",
        help = "Move files instead of copying them",
        group = "action"
    )]
    move_arg: bool,
    // Sorting
    #[arg(
        short,
        long,
        help = "Sort values by size, extension(ext) or creation_date(date)"
    )]
    sort_by: SortBy,
    #[arg(
        short = 'a',
        long = "all",
        help = "Include hidden files (dotfiles) in sorting"
    )]
    search_hidden: bool,
    #[arg(
        short = 'r',
        long = "recursive",
        help = "Sort directories recursively. All inner files will be sorted as well"
    )]
    search_recursive: bool,
}

// Directory iterator
//  -- Sort files in directory
//  -- Return inner directories paths
//  -- Repeat for each inner directory if recursive
// Sorter command
//  -- Collect all paths for directory
//  -- Sort paths by criteria
//  -- Move/copy files
//  -- Return inner directories paths with new locations
impl SortCommand {
    pub fn execute(&self, _context: &Cli) -> anyhow::Result<()> {
        debug!("Executing 'SORT' command");
        trace!("with configuration: {self}");

        let _file_action = FileAction::from(self);
        let mut paths = vec![self.directory.clone()];
        while let Some(path) = paths.pop() {
            let files_list = FilesList::new(&path, false, self.search_hidden)?;

            debug!("Sorting directory: {path:?}");
            let (_sorted_files_list, _inner_directories_paths) =
                sort_directory(files_list, self.sort_by)?;

            // debug!("Files sorted, transferring files");
            // let new_inner_directories_paths = transfer_files(
            //     sorted_files_list,
            //     inner_directories_paths,
            //     &path,
            //     &file_action,
            // )?;

            // paths.extend(new_inner_directories_paths);
        }

        Ok(())
    }
}

impl Display for SortCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}: {}",
            "directory".bright_cyan(),
            self.directory.to_string_lossy()
        )?;
        writeln!(f, "{}: {}", "copy".bright_cyan(), self.copy)?;
        writeln!(f, "{}: {}", "move".bright_cyan(), self.move_arg)?;
        writeln!(f, "{}: {:?}", "sort_by".bright_cyan(), self.sort_by)?;
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
