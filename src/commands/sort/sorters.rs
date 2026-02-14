use std::{collections::HashMap, fmt::Display, fs::DirEntry};

use time::OffsetDateTime;

pub fn sort_by_extension(files: Vec<DirEntry>) -> anyhow::Result<HashMap<String, Vec<DirEntry>>> {
    let mut sorted_files = HashMap::new();

    for file in files {
        let extension = match file.path().extension() {
            Some(ext) => ext.to_string_lossy().into_owned(),
            None => "no_extension".to_string(),
        };

        sorted_files
            .entry(extension)
            .or_insert_with(Vec::new)
            .push(file);
    }

    Ok(sorted_files)
}

pub fn sort_by_date(files: Vec<DirEntry>) -> anyhow::Result<HashMap<String, Vec<DirEntry>>> {
    let mut sorted_files = HashMap::new();

    for file in files {
        let created_time = file.metadata()?.created()?;
        let date = format!("{:?}", OffsetDateTime::from(created_time).date());

        sorted_files.entry(date).or_insert_with(Vec::new).push(file);
    }

    Ok(sorted_files)
}

pub fn sort_by_size(files: Vec<DirEntry>) -> anyhow::Result<HashMap<String, Vec<DirEntry>>> {
    let mut sorted_files = HashMap::new();

    for file in files {
        let file_bytes = file.metadata()?.len();
        let size = FileSize::from_bytes(file_bytes).to_string();

        sorted_files.entry(size).or_insert_with(Vec::new).push(file);
    }

    Ok(sorted_files)
}

#[derive(Debug, PartialEq, Eq)]
enum FileSize {
    KB(u64),
    MB(u64),
    GB(u64),
}

impl FileSize {
    fn from_bytes(bytes: u64) -> Self {
        if bytes < 1_048_576 {
            FileSize::KB(bytes / 1024)
        } else if bytes < 1_073_741_824 {
            FileSize::MB(bytes / 1024 / 1024)
        } else {
            FileSize::GB(bytes / 1024 / 1024 / 1024)
        }
    }
}

impl Display for FileSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileSize::KB(size) => write!(f, "{}-{} KB", size / 100 * 100, size / 100 * 100 + 99),
            FileSize::MB(size) => write!(f, "{}-{} MB", size / 100 * 100, size / 100 * 100 + 99),
            FileSize::GB(size) => write!(f, "{} GB", size),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{commands::sort::FilesList, volumes::get_path_under_tests};

    use super::*;
    use test_case::test_case;

    #[test]
    fn test_sort_by_extension() {
        let file_list = FilesList::new(
            &get_path_under_tests("sorters/sort_by_extension"),
            false,
            false,
        )
        .unwrap();
        let files = file_list
            .into_iter()
            .map(|res| res.unwrap())
            .collect::<Vec<DirEntry>>();

        let sorted_files = sort_by_extension(files).unwrap();
        assert_eq!(sorted_files.len(), 3);
        assert!(sorted_files.contains_key("txt"));
        assert!(sorted_files.contains_key("rs"));
        assert!(sorted_files.contains_key("no_extension"));
    }

    // Note: This test relies on the file creation time, that can be tricky to set up in a test environment.
    // If you want to test this, you should change "2026-02-14" to the actual creation date of the file in the "sorters/sort_by_date" directory.
    // fn test_sort_by_date() {
    //     let file_list =
    //         FilesList::new(&get_path_under_tests("sorters/sort_by_date"), false, false).unwrap();
    //     let files = file_list
    //         .into_iter()
    //         .map(|res| res.unwrap())
    //         .collect::<Vec<DirEntry>>();

    //     let sorted_files = sort_by_date(files).unwrap();
    //     assert_eq!(sorted_files.len(), 1);
    //     assert!(sorted_files.contains_key("2026-02-14"));
    // }

    #[test]
    fn test_sort_by_size() {
        let file_list =
            FilesList::new(&get_path_under_tests("sorters/sort_by_size"), false, false).unwrap();
        let files = file_list
            .into_iter()
            .map(|res| res.unwrap())
            .collect::<Vec<DirEntry>>();

        let sorted_files = sort_by_size(files).unwrap();
        assert_eq!(sorted_files.len(), 2);
        assert!(sorted_files.contains_key("0-99 KB"));
        assert!(sorted_files.contains_key("100-199 KB"));
    }

    #[test_case(1023, FileSize::KB(0))]
    #[test_case(1024, FileSize::KB(1))]
    #[test_case(1_048_575, FileSize::KB(1023))]
    #[test_case(1_048_576, FileSize::MB(1))]
    #[test_case(1_073_741_823, FileSize::MB(1023))]
    #[test_case(1_073_741_824, FileSize::GB(1))]
    #[test_case(10_737_418_240, FileSize::GB(10))]
    fn test_file_size_from_bytes(size: u64, expected: FileSize) {
        let file_size = FileSize::from_bytes(size);
        assert_eq!(file_size, expected);
    }

    #[test_case(FileSize::KB(1), "0-99 KB")]
    #[test_case(FileSize::KB(500), "500-599 KB")]
    #[test_case(FileSize::KB(1023), "1000-1099 KB")]
    #[test_case(FileSize::MB(1), "0-99 MB")]
    #[test_case(FileSize::MB(500), "500-599 MB")]
    #[test_case(FileSize::MB(1023), "1000-1099 MB")]
    #[test_case(FileSize::GB(1), "1 GB")]
    #[test_case(FileSize::GB(10), "10 GB")]
    fn test_file_size_display(size: FileSize, expected: &str) {
        let display = size.to_string();
        assert_eq!(display, expected);
    }
}
