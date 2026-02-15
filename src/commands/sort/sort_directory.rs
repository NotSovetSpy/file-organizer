use std::{collections::HashMap, fs::DirEntry};

use anyhow::bail;

use super::FilesList;
use crate::commands::sort::SortBy;

// Key is a directory name (e.g. "txt" for extension or "1KB-10KB" for size), value is a list of files in that directory
pub type SortedFiles = HashMap<String, Vec<DirEntry>>;

pub fn sort_directory(files_list: FilesList, sort_by: SortBy) -> anyhow::Result<SortedFiles> {
    let files = collect_directories(files_list)?;
    let sorted_files = sort_by.sort(files)?;
    Ok(sorted_files)
}

fn collect_directories(files_list: FilesList) -> anyhow::Result<Vec<DirEntry>> {
    let mut files = Vec::new();

    for file in files_list.into_iter() {
        match file {
            Ok(entry) => {
                files.push(entry);
            }
            Err(e) => {
                bail!("Error reading file: {e}");
            }
        }
    }

    Ok(files)
}

#[cfg(test)]
mod tests {
    use crate::{commands::sort::FilesList, volumes::get_path_under_tests};

    #[test]
    fn test_collect_directories() {
        let file_list =
            FilesList::new(&get_path_under_tests("sort_directory"), false, false).unwrap();
        let files = super::collect_directories(file_list).unwrap();
        let files_names: Vec<String> = files
            .into_iter()
            .map(|entry| entry.file_name().to_string_lossy().to_string())
            .collect();

        assert_eq!(files_names.len(), 4);
        assert!(files_names.contains(&"file1.txt".to_string()));
        assert!(files_names.contains(&"file2.txt".to_string()));
        assert!(files_names.contains(&"inner_dir_1".to_string()));
        assert!(files_names.contains(&"inner_dir_2".to_string()));
    }
}
