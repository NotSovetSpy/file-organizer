use crate::{
    cli::Cli,
    commands::{arguments::FindArguments, find::file_list::FileList},
};

mod file_list;

pub fn execute(args: FindArguments, _context: &Cli) -> anyhow::Result<()> {
    let files = FileList::new(args.directory, args.is_recursive, args.search_hidden)?;
    // filtering
    // printing
    Ok(())
}
