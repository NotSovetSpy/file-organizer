use crate::cli::Cli;
#[allow(dead_code)]

pub struct FindArguments<'a> {
    directory: &'a String,
    file_name: Option<&'a String>,
    size: Option<&'a String>,
    ext: Option<&'a String>,
    date: Option<&'a String>,
    search_hidden: bool,
    output: &'a String,
    is_regex: bool,
    is_recursive: bool,
}

impl<'a> FindArguments<'a> {
    pub fn new(
        directory: &'a String,
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

pub fn execute(_args: FindArguments, _context: &Cli) -> anyhow::Result<()> {
    Ok(())
}
