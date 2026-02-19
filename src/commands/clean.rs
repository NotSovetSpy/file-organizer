use std::{fmt::Display, fs, path::PathBuf};

use clap::Parser;
use log::{debug, info, trace, warn};
use owo_colors::OwoColorize;

use crate::cli::Cli;
use crate::confirmation::confirm;

pub(super) use super::find::FilesList;

/// File name patterns to delete (checked via glob-like matching)
const JUNK_FILE_PATTERNS: &[&str] = &[".DS_Store", "Thumbs.db", "desktop.ini"];

/// File extensions to delete (without the leading dot)
const JUNK_EXTENSIONS: &[&str] = &[
    "tmp",
    "temp",
    "swp",
    "swo",
    "bak",
    "old",
    "autosave",
    "log",
    "core",
    "dmp",
    "stackdump",
    "pyc",
    "pyo",
    "class",
    "ilk",
];

/// Prefixes/suffixes checked separately:
/// - files ending with `~`
/// - files matching `*.log.*`
///
/// Directory names to delete entirely
const JUNK_DIR_NAMES: &[&str] = &[
    "tmp",
    "temp",
    ".cache",
    "cache",
    "Cache",
    "Caches",
    "__pycache__",
    "target",
    "dist",
    "build",
    "out",
    ".gradle",
    ".pytest_cache",
    ".mypy_cache",
    ".ruff_cache",
    ".parcel-cache",
    ".vite",
    ".next",
    ".nuxt",
];

#[derive(Parser, Debug, Default)]
pub struct CleanCommand {
    #[arg(help = "Directory to clean", default_value = ".")]
    directory: PathBuf,
    #[arg(
        short = 'a',
        long = "all",
        help = "Include hidden files (dotfiles) in the search"
    )]
    search_hidden: bool,
    #[arg(
        short = 'r',
        long = "recursive",
        help = "Search directories recursively"
    )]
    search_recursive: bool,
}

impl CleanCommand {
    pub fn execute(&self, _context: &Cli) -> anyhow::Result<()> {
        debug!("Executing 'clean' command");
        trace!("with configuration: {self}");

        if !confirm(
            "This command will permanently delete junk files and directories. Make sure that you have closed all applications that might be using these files. Are you sure you want to proceed?",
        ) {
            debug!("User declined to delete files. Aborting command execution.");
            return Ok(());
        }

        let files = FilesList::new(&self.directory, self.search_recursive, self.search_hidden)?;

        let mut deleted_files: u64 = 0;
        let mut deleted_dirs: u64 = 0;
        let mut total_bytes: u64 = 0;

        for entry in files {
            let entry = entry?;
            let metadata = entry.metadata()?;
            let name = entry.file_name().to_string_lossy().into_owned();

            if metadata.is_dir() {
                if Self::is_junk_dir(&name) {
                    let path = entry.path();
                    let dir_size = dir_size_recursive(&path);
                    info!("{} {}", "Deleting directory:".bright_red(), path.display());
                    match fs::remove_dir_all(&path) {
                        Ok(()) => {
                            deleted_dirs += 1;
                            total_bytes += dir_size;
                        }
                        Err(e) => {
                            warn!("Failed to delete directory {}: {}", path.display(), e);
                        }
                    }
                }
            } else if Self::is_junk_file(&name) {
                let path = entry.path();
                let size = metadata.len();
                info!("{} {}", "Deleting file:".bright_red(), path.display());
                match fs::remove_file(&path) {
                    Ok(()) => {
                        deleted_files += 1;
                        total_bytes += size;
                    }
                    Err(e) => {
                        warn!("Failed to delete file {}: {}", path.display(), e);
                    }
                }
            }
        }

        println!(
            "\n{}\n  {} file(s) deleted\n  {} directory(ies) deleted\n  {} freed",
            "Clean summary:".bold(),
            deleted_files.bright_green(),
            deleted_dirs.bright_green(),
            format_bytes(total_bytes).bright_green(),
        );

        Ok(())
    }

    fn is_junk_file(name: &str) -> bool {
        // Exact name match
        if JUNK_FILE_PATTERNS.contains(&name) {
            return true;
        }

        // Files ending with ~
        if name.ends_with('~') {
            return true;
        }

        // Extension match (e.g. *.tmp, *.bak, etc.)
        if let Some(ext) = name.rsplit('.').next()
            && JUNK_EXTENSIONS.contains(&ext)
        {
            return true;
        }

        // *.log.* pattern (e.g. app.log.1, error.log.2023-01-01)
        if name.contains(".log.") {
            return true;
        }

        false
    }

    fn is_junk_dir(name: &str) -> bool {
        JUNK_DIR_NAMES.contains(&name)
    }
}

impl Display for CleanCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}: {}",
            "directory".bright_cyan(),
            self.directory.display()
        )?;
        writeln!(
            f,
            "{}: {}",
            "search_hidden".bright_cyan(),
            self.search_hidden
        )?;
        writeln!(
            f,
            "{}: {}",
            "search_recursive".bright_cyan(),
            self.search_recursive
        )?;
        Ok(())
    }
}

/// Recursively calculate directory size
fn dir_size_recursive(path: &PathBuf) -> u64 {
    let mut total: u64 = 0;
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            if let Ok(meta) = entry.metadata() {
                if meta.is_dir() {
                    total += dir_size_recursive(&entry.path().to_path_buf());
                } else {
                    total += meta.len();
                }
            }
        }
    }
    total
}

/// Format bytes into human-readable string
fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = 1024 * KB;
    const GB: u64 = 1024 * MB;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{bytes} B")
    }
}

#[cfg(test)]
mod tests {
    use super::CleanCommand;

    #[test]
    fn test_is_junk_file_exact_names() {
        assert!(CleanCommand::is_junk_file(".DS_Store"));
        assert!(CleanCommand::is_junk_file("Thumbs.db"));
        assert!(CleanCommand::is_junk_file("desktop.ini"));
    }

    #[test]
    fn test_is_junk_file_extensions() {
        assert!(CleanCommand::is_junk_file("file.tmp"));
        assert!(CleanCommand::is_junk_file("data.temp"));
        assert!(CleanCommand::is_junk_file(".file.swp"));
        assert!(CleanCommand::is_junk_file(".file.swo"));
        assert!(CleanCommand::is_junk_file("backup.bak"));
        assert!(CleanCommand::is_junk_file("config.old"));
        assert!(CleanCommand::is_junk_file("draft.autosave"));
        assert!(CleanCommand::is_junk_file("app.log"));
        assert!(CleanCommand::is_junk_file("core.core"));
        assert!(CleanCommand::is_junk_file("crash.dmp"));
        assert!(CleanCommand::is_junk_file("crash.stackdump"));
        assert!(CleanCommand::is_junk_file("module.pyc"));
        assert!(CleanCommand::is_junk_file("module.pyo"));
        assert!(CleanCommand::is_junk_file("Main.class"));
        assert!(CleanCommand::is_junk_file("lib.ilk"));
    }

    #[test]
    fn test_is_junk_file_tilde() {
        assert!(CleanCommand::is_junk_file("document~"));
        assert!(CleanCommand::is_junk_file("file.txt~"));
    }

    #[test]
    fn test_is_junk_file_log_dot() {
        assert!(CleanCommand::is_junk_file("app.log.1"));
        assert!(CleanCommand::is_junk_file("error.log.2023-01-01"));
    }

    #[test]
    fn test_is_not_junk_file() {
        assert!(!CleanCommand::is_junk_file("main.rs"));
        assert!(!CleanCommand::is_junk_file("Cargo.toml"));
        assert!(!CleanCommand::is_junk_file("README.md"));
        assert!(!CleanCommand::is_junk_file("data.json"));
    }

    #[test]
    fn test_is_junk_dir() {
        assert!(CleanCommand::is_junk_dir("tmp"));
        assert!(CleanCommand::is_junk_dir("temp"));
        assert!(CleanCommand::is_junk_dir(".cache"));
        assert!(CleanCommand::is_junk_dir("cache"));
        assert!(CleanCommand::is_junk_dir("Cache"));
        assert!(CleanCommand::is_junk_dir("Caches"));
        assert!(CleanCommand::is_junk_dir("__pycache__"));
        assert!(CleanCommand::is_junk_dir("target"));
        assert!(CleanCommand::is_junk_dir("dist"));
        assert!(CleanCommand::is_junk_dir("build"));
        assert!(CleanCommand::is_junk_dir("out"));
        assert!(CleanCommand::is_junk_dir(".gradle"));
        assert!(CleanCommand::is_junk_dir(".pytest_cache"));
        assert!(CleanCommand::is_junk_dir(".mypy_cache"));
        assert!(CleanCommand::is_junk_dir(".ruff_cache"));
        assert!(CleanCommand::is_junk_dir(".parcel-cache"));
        assert!(CleanCommand::is_junk_dir(".vite"));
        assert!(CleanCommand::is_junk_dir(".next"));
        assert!(CleanCommand::is_junk_dir(".nuxt"));
    }

    #[test]
    fn test_is_not_junk_dir() {
        assert!(!CleanCommand::is_junk_dir("src"));
        assert!(!CleanCommand::is_junk_dir("lib"));
        assert!(!CleanCommand::is_junk_dir("tests"));
    }

    #[test]
    fn test_format_bytes() {
        use super::format_bytes;
        assert_eq!(format_bytes(0), "0 B");
        assert_eq!(format_bytes(512), "512 B");
        assert_eq!(format_bytes(1024), "1.00 KB");
        assert_eq!(format_bytes(1536), "1.50 KB");
        assert_eq!(format_bytes(1048576), "1.00 MB");
        assert_eq!(format_bytes(1073741824), "1.00 GB");
    }
}
