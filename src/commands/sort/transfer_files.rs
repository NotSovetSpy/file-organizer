use std::{fs::DirEntry, path::PathBuf};

use crate::commands::sort::file_action::FileAction;

pub fn transfer_files(
    sorted_files_list: Vec<DirEntry>,
    inner_directories_paths: Vec<PathBuf>,
    current_directory_path: &PathBuf,
    file_action: &FileAction,
) -> anyhow::Result<Vec<PathBuf>> {
    todo!()
}
