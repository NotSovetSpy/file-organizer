use std::path::Path;

use crate::commands::sort::SortCommand;

#[derive(Clone, Debug, Copy)]
pub enum FileAction {
    Move,
    Copy,
}

impl From<&SortCommand> for FileAction {
    fn from(command: &SortCommand) -> Self {
        if command.copy {
            FileAction::Copy
        } else if command.move_arg {
            FileAction::Move
        } else {
            panic!(
                "Should never happened, because we set group for copy and move arguments, so one of them should be always set"
            )
        }
    }
}

pub type FileActionFn = Box<dyn Fn(&Path, &Path) -> std::io::Result<()>>;

impl FileAction {
    pub fn get_action_fn(&self) -> FileActionFn {
        match self {
            FileAction::Move => Box::new(|src: &Path, dst: &Path| std::fs::rename(src, dst)),
            FileAction::Copy => {
                Box::new(|src: &Path, dst: &Path| std::fs::copy(src, dst).map(|_| ()))
            }
        }
    }
}
