use clap::Subcommand;

use crate::{cli::Cli, commands::find::FindArguments};

mod display;
mod find;

#[derive(Subcommand, Debug)]
pub enum Commands {
    Find {
        directory: String,
        // TODO: Implement
        #[arg(short = 'n', long, name = "name")]
        file_name: Option<String>,
        // TODO: Implement
        // TODO: Consider change value type
        #[arg(short, long)]
        size: Option<String>,
        // TODO: Implement
        // TODO: Consider change value type
        #[arg(short, long)]
        ext: Option<String>,
        // TODO: Implement
        // TODO: Consider change value type
        #[arg(short, long)]
        date: Option<String>,
        // TODO: Implement
        #[arg(short = 'a', long = "all")]
        search_hidden: bool,
        // TODO: Implement
        // TODO: Consider change value type
        #[arg(short, long, default_value_t = String::from("console"))]
        output: String,
        #[arg(long = "regex")]
        is_regex: bool,
        #[arg(short = 'r', long = "recursive", default_value_t = true)]
        is_recursive: bool,
    },
}

impl Commands {
    pub fn execute(&self, context: &Cli) -> anyhow::Result<()> {
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
                let args = FindArguments::new(
                    directory,
                    file_name.as_ref(),
                    size.as_ref(),
                    ext.as_ref(),
                    date.as_ref(),
                    *search_hidden,
                    output,
                    *is_regex,
                    *is_recursive,
                );
                find::execute(args, context)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::panic::catch_unwind;

    use crate::cli::Cli;
    use clap::Parser;

    #[test]
    fn test_find_configuration_syntax() {
        let args = vec!["fo", "find", "/tmp"];
        let is_panic_result = catch_unwind(|| Cli::try_parse_from(args));
        assert!(is_panic_result.is_ok());
    }
}
