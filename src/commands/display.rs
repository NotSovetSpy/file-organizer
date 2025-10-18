use std::fmt::Display;

use owo_colors::OwoColorize;

use crate::commands::Commands;

impl Display for Commands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Commands::Find {
                directory,
                file_name,
                size,
                ext,
                date,
                search_hidden,
                output,
                is_regex,
                is_recursive,
            } => {
                writeln!(f, "{}: {}", "command_name".bright_cyan(), "find")?;
                writeln!(
                    f,
                    "{}: {}",
                    "filename".bright_cyan(),
                    file_name.as_ref().unwrap_or(&String::from("None"))
                )?;
                writeln!(f, "{}: {}", "directory".bright_cyan(), directory)?;
                writeln!(
                    f,
                    "{}: {}",
                    "size".bright_cyan(),
                    size.as_ref().unwrap_or(&String::from("None"))
                )?;
                writeln!(
                    f,
                    "{}: {}",
                    "ext".bright_cyan(),
                    ext.as_ref().unwrap_or(&String::from("None"))
                )?;
                writeln!(
                    f,
                    "{}: {}",
                    "date".bright_cyan(),
                    date.as_ref().unwrap_or(&String::from("None"))
                )?;
                writeln!(f, "{}: {}", "search_hidden".bright_cyan(), search_hidden)?;
                writeln!(f, "{}: {}", "output".bright_cyan(), output)?;
                writeln!(f, "{}: {}", "regex".bright_cyan(), is_regex)?;
                writeln!(f, "{}: {}", "recursive".bright_cyan(), is_recursive)?;
                Ok(())
            }
        }
    }
}
