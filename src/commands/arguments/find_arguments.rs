use std::path::PathBuf;

#[allow(dead_code)]

pub struct FindArguments<'a> {
    pub directory: &'a PathBuf,
    pub file_name: Option<&'a String>,
    pub size: Option<&'a String>,
    pub ext: Option<&'a String>,
    pub date: Option<&'a String>,
    pub search_hidden: bool,
    pub output: &'a String,
    pub is_regex: bool,
    pub is_recursive: bool,
}

impl<'a> FindArguments<'a> {
    pub fn new(
        directory: &'a PathBuf,
        file_name: Option<&'a String>,
        size: Option<&'a String>,
        ext: Option<&'a String>,
        date: Option<&'a String>,
        search_hidden: bool,
        output: &'a String,
        is_regex: bool,
        is_recursive: bool,
    ) -> Self {
        FindArguments {
            directory,
            file_name,
            size,
            ext,
            date,
            search_hidden,
            output,
            is_regex,
            is_recursive,
        }
    }
}
