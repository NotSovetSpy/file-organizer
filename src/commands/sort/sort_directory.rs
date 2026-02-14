use std::{collections::HashMap, fs::DirEntry, path::PathBuf};

use anyhow::bail;

use super::FilesList;
use crate::commands::sort::SortBy;

// Key is a directory name (e.g. "txt" for extension or "1KB-10KB" for size), value is a list of files in that directory
pub type SortedFiles = HashMap<String, Vec<DirEntry>>;

pub fn sort_directory(
    files_list: FilesList,
    sort_by: SortBy,
) -> anyhow::Result<(SortedFiles, Vec<PathBuf>)> {
    let (files, inner_directories_paths) = collect_directories(files_list)?;
    let sorted_files = sort_by.sort(files)?;
    Ok((sorted_files, inner_directories_paths))
}

fn collect_directories(files_list: FilesList) -> anyhow::Result<(Vec<DirEntry>, Vec<PathBuf>)> {
    let mut files = Vec::new();
    let mut inner_directories_paths = Vec::new();

    for file in files_list.into_iter() {
        match file {
            Ok(entry) => {
                if entry.file_type()?.is_dir() {
                    inner_directories_paths.push(entry.path());
                } else {
                    files.push(entry);
                }
            }
            Err(e) => {
                bail!("Error reading file: {e}");
            }
        }
    }

    Ok((files, inner_directories_paths))
}
