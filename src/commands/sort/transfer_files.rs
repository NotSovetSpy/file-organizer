use std::{
    fs::{self},
    path::{Path, PathBuf},
};

use log::trace;

use crate::commands::sort::{
    file_action::{FileAction, FileActionFn},
    sort_directory::SortedFiles,
};

pub fn transfer_files(
    sorted_files_list: SortedFiles,
    target_root_path: &Path,
    file_action: &FileAction,
) -> anyhow::Result<Vec<PathBuf>> {
    let action_fn = file_action.get_action_fn();
    let mut inner_directories_paths = Vec::new();

    for (sorted_directory, files) in sorted_files_list {
        let sorted_dir_path = target_root_path.join(sorted_directory);
        trace!("Created directory: {sorted_dir_path:?}");
        fs::create_dir(&sorted_dir_path)?;

        for file in files {
            let file_name = file.file_name();
            let target_file_path = sorted_dir_path.join(file_name);

            if file.file_type()?.is_dir() {
                trace!(
                    "Recursively applying file_action for directory: {:?} to {:?}",
                    file.path(),
                    target_file_path
                );
                // Recursively apply file_action for directory
                apply_file_action_recursive(&file.path(), &target_file_path, &action_fn)?;
                inner_directories_paths.push(target_file_path.clone());
            } else {
                trace!(
                    "Transferring file: {:?} to {:?}",
                    file.path(),
                    target_file_path
                );
                action_fn(&file.path(), &target_file_path)?;
            }
        }
    }

    Ok(inner_directories_paths)
}

fn apply_file_action_recursive(
    src: &Path,
    dst: &Path,
    action_fn: &FileActionFn,
) -> anyhow::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let entry_path = entry.path();
        let target_path = dst.join(entry.file_name());
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            apply_file_action_recursive(&entry_path, &target_path, action_fn)?;
        } else {
            action_fn(&entry_path, &target_path)?;
        }
    }
    Ok(())
}
