use std::{fs::DirEntry, path::PathBuf};

use crate::commands::sort::file_action::FileAction;

pub fn transfer_files(
    _sorted_files_list: Vec<DirEntry>,
    _inner_directories_paths: Vec<PathBuf>,
    _current_directory_path: &PathBuf,
    _file_action: &FileAction,
) -> anyhow::Result<Vec<PathBuf>> {
    todo!()
}
