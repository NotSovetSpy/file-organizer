use std::{
    fs::{DirEntry, ReadDir, read_dir},
    path::PathBuf,
};

/// Iterator over files in directory with extra specific iterating rules
pub struct FilesList {
    is_recursive: bool,
    search_hidden: bool,
    // Represent directory stack from start directory, that dynamically add dirs by DFS algorithm
    dir_stack: Vec<ReadDir>,
}

impl FilesList {
    pub fn new(
        start_directory: &PathBuf,
        is_recursive: bool,
        search_hidden: bool,
    ) -> anyhow::Result<Self> {
        let root_dir = read_dir(start_directory)?;

        Ok(FilesList {
            is_recursive,
            search_hidden,
            dir_stack: Vec::from([root_dir]),
        })
    }
}

impl Iterator for FilesList {
    type Item = anyhow::Result<DirEntry>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let directory = match self.dir_stack.last_mut() {
                Some(directory) => directory,
                None => break None,
            };

            match directory.next() {
                Some(result) => match result {
                    Ok(file) => {
                        let metadata = match file.metadata() {
                            Ok(metadata) => metadata,
                            Err(err) => break Some(Err(err.into())),
                        };

                        // Skip hidden files if search_hidden flag not enabled
                        if !self.search_hidden
                            && file.file_name().to_string_lossy().chars().next() == Some('.')
                        {
                            continue;
                        }

                        // If recursive flag enabled, add new directory to stack to follow DFS algorithm
                        if self.is_recursive && metadata.is_dir() {
                            let dir = match read_dir(file.path()) {
                                Ok(dir) => dir,
                                Err(err) => break Some(Err(err.into())),
                            };
                            self.dir_stack.push(dir);
                        }

                        // Return read file
                        break Some(Ok(file));
                    }
                    Err(err) => break Some(Err(err.into())),
                },
                None => {
                    // Remove read directory from stack
                    self.dir_stack.pop();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{commands::find::file_list::FilesList, volumes::get_path_under_tests};

    #[test]
    fn test_iterate_file_list_without_recursion() {
        let file_list = FilesList::new(&get_path_under_tests("file_list"), false, false).unwrap();

        let mut result = HashSet::new();
        for file in file_list {
            result.insert(file.unwrap().file_name().to_string_lossy().into_owned());
        }

        let expected = HashSet::from(["a".to_string(), "d".to_string(), "i".to_string()]);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_iterate_file_list_with_recursion() {
        let file_list = FilesList::new(&get_path_under_tests("file_list"), true, false).unwrap();

        let mut result = HashSet::new();
        for file in file_list {
            result.insert(file.unwrap().file_name().to_string_lossy().into_owned());
        }

        let expected = HashSet::from([
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
            "e".to_string(),
            "f".to_string(),
            "g".to_string(),
            "h".to_string(),
            "i".to_string(),
        ]);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_iterate_file_list_without_hidden() {
        let file_list = FilesList::new(&get_path_under_tests("file_list"), true, false).unwrap();

        let mut result = HashSet::new();
        for file in file_list {
            result.insert(file.unwrap().file_name().to_string_lossy().into_owned());
        }

        let expected = HashSet::from([
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
            "e".to_string(),
            "f".to_string(),
            "g".to_string(),
            "h".to_string(),
            "i".to_string(),
        ]);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_iterate_file_list_with_hidden() {
        let file_list = FilesList::new(&get_path_under_tests("file_list"), true, true).unwrap();

        let mut result = HashSet::new();
        for file in file_list {
            result.insert(file.unwrap().file_name().to_string_lossy().into_owned());
        }

        let expected = HashSet::from([
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
            "e".to_string(),
            "f".to_string(),
            "g".to_string(),
            "h".to_string(),
            "i".to_string(),
            ".hidden_a".to_string(),
            ".hidden_b".to_string(),
            ".hidden_c".to_string(),
            ".hidden_d".to_string(),
        ]);

        assert_eq!(result, expected);
    }
}
