use crate::commands::sort::SortCommand;

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
