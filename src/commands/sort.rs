use std::{
    fmt::{Debug, Display},
    fs,
    path::{Path, PathBuf},
};

use clap::Parser;
use log::{debug, trace};

use crate::{cli::Cli, confirmation::confirm};
use owo_colors::OwoColorize;
use sort_directory::sort_directory;
use transfer_files::transfer_files;

mod file_action;
mod sort_by;
mod sort_directory;
mod sorters;
mod transfer_files;

pub(super) use super::find::FilesList;
pub(super) use file_action::FileAction;
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

        if self.move_arg
            && !confirm(
                "You have chosen to move files. Are you sure you want to proceed? This action cannot be undone.",
            )
        {
            debug!("User declined to move files. Aborting command execution.");
            return Ok(());
        }

        let mut file_action = FileAction::from(self);
        let mut paths = vec![self.directory.clone()];

        self.create_target_root_directory(&self.directory)?;

        while let Some(path) = paths.pop() {
            let files_list = FilesList::new(&path, false, self.search_hidden)?;
            let target_root_path = if path == self.directory {
                &self.directory.join(format!(
                    "../{}_sorted",
                    self.directory
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                ))
            } else {
                &path
            };

            debug!("Sorting directory: {path:?}");
            let sorted_files_list = sort_directory(files_list, self.sort_by)?;

            debug!("Transferring files from directory: {path:?}");
            let inner_directories_paths =
                transfer_files(sorted_files_list, target_root_path, &file_action)?;

            if self.search_recursive {
                paths.extend(inner_directories_paths);
                file_action = FileAction::Move;
            }
        }

        Ok(())
    }

    fn create_target_root_directory(&self, current_directory_path: &Path) -> anyhow::Result<()> {
        let source_directory_name = current_directory_path
            .file_name()
            .unwrap_or_else(|| std::ffi::OsStr::new(""));
        let target_root_path = current_directory_path.join(format!(
            "../{}_sorted",
            source_directory_name.to_string_lossy()
        ));
        trace!("Creating target root directory: {target_root_path:?}");
        fs::create_dir(&target_root_path)?;
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
