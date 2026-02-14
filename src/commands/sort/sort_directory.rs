use std::{fs::DirEntry, path::PathBuf};

use super::FilesList;
use crate::commands::sort::SortBy;

pub fn sort_directory(
    _files_list: FilesList,
    _sort_by: &SortBy,
) -> anyhow::Result<(Vec<DirEntry>, Vec<PathBuf>)> {
    todo!()
}
