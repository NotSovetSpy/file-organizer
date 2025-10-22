mod extractors;
mod filters;

use crate::{cli::Cli, commands::find::FindCommand};
use anyhow::bail;
pub use extractors::*;
use filters::*;
use std::fs::DirEntry;
use time::PrimitiveDateTime;

/// Trait for matching files against filters
pub trait FileMatcherTrait {
    fn matches(&self, file: &DirEntry, cli: &Cli) -> anyhow::Result<bool>;
}

/// Main structure that combines file value extraction and filtering
/// FilterValue - type that extractor returns and filter accepts
/// E - extractor that implements Extractor trait
pub struct FileMatcher<FilterValue, E>
where
    E: Extractor<FilterValue>,
{
    filter: Box<dyn Filter<FilterValue>>,
    extractor: E,
}

impl<FilterValue, E> FileMatcherTrait for FileMatcher<FilterValue, E>
where
    FilterValue: 'static,
    E: Extractor<FilterValue>,
{
    fn matches(&self, file: &DirEntry, cli: &Cli) -> anyhow::Result<bool> {
        let value = self.extractor.extract(file, cli)?;
        Ok(self.filter.is_matched(value))
    }
}

impl<FilterValue, E> FileMatcher<FilterValue, E>
where
    FilterValue: 'static,
    E: Extractor<FilterValue>,
{
    /// Create a FileMatcher with an exact match filter
    pub fn with_exact_match(compare_value: FilterValue, extractor: E) -> Self
    where
        FilterValue: PartialEq + 'static,
    {
        Self {
            filter: Box::new(ExactMatchFilter::new(compare_value)),
            extractor,
        }
    }
}

impl<E> FileMatcher<String, E>
where
    E: Extractor<String>,
{
    /// Create a FileMatcher with a regex filter (only for String extractors)
    pub fn with_regex(regex_pattern: &String, extractor: E) -> anyhow::Result<Self> {
        Ok(Self {
            filter: Box::new(RegexFilter::new(regex_pattern)?),
            extractor,
        })
    }
}

pub fn create_matcher_from_config(
    config: &FindCommand,
    context: &Cli,
) -> anyhow::Result<Box<dyn FileMatcherTrait>> {
    if let Some(file_name) = &config.file_name {
        if !config.is_regex {
            let matcher = FileMatcher::with_exact_match(file_name.clone(), FileNameExtractor);
            Ok(Box::new(matcher))
        } else {
            let matcher = FileMatcher::with_regex(file_name, FileNameExtractor)?;
            Ok(Box::new(matcher))
        }
    } else if let Some(size) = &config.size {
        if !config.is_regex {
            let number = size.parse::<u64>().map_err(|_| {
                anyhow::anyhow!("Size must be a valid number if regex is not enabled")
            })?;
            let matcher = FileMatcher::with_exact_match(number, FileSizeExtractor);
            Ok(Box::new(matcher))
        } else {
            let matcher = FileMatcher::with_regex(&size.to_string(), FileSizeRegexExtractor)?;
            Ok(Box::new(matcher))
        }
    } else if let Some(ext) = &config.ext {
        if !config.is_regex {
            let matcher = FileMatcher::with_exact_match(ext.clone(), FileExtensionExtractor);
            Ok(Box::new(matcher))
        } else {
            let matcher = FileMatcher::with_regex(ext, FileExtensionExtractor)?;
            Ok(Box::new(matcher))
        }
    } else if let Some(datetime) = &config.datetime {
        if !config.is_regex {
            let matcher = FileMatcher::with_exact_match(
                PrimitiveDateTime::parse(datetime, &context.datetime_format)?,
                FileDateExtractor,
            );
            Ok(Box::new(matcher))
        } else {
            let matcher = FileMatcher::with_regex(datetime, FileDateRegexExtractor)?;
            Ok(Box::new(matcher))
        }
    } else if let Some(modified) = &config.modified {
        if !config.is_regex {
            let matcher = FileMatcher::with_exact_match(
                PrimitiveDateTime::parse(modified, &context.datetime_format)?,
                FileDateExtractor,
            );
            Ok(Box::new(matcher))
        } else {
            let matcher = FileMatcher::with_regex(modified, FileDateRegexExtractor)?;
            Ok(Box::new(matcher))
        }
    } else {
        bail!("No valid filter configuration found");
    }
}

#[cfg(test)]
mod tests {
    // It's hard to unit test filtering because of high abstraction level (many traits and generics are involved).
    // So, most practical way is to do by testing each possible File matcher from create_matcher_from_config function.
    // It mostly test not create_matcher_from_config function itself, but the involved combination of extractors and filters.

    use crate::{
        cli::Cli, commands::find::filtering::create_matcher_from_config,
        volumes::get_path_under_tests,
    };
    use time::PrimitiveDateTime;

    fn get_dir_entry_from_path(path_str: &str) -> std::fs::DirEntry {
        let path = get_path_under_tests(path_str);
        std::fs::read_dir(path).unwrap().next().unwrap().unwrap()
    }

    #[test]
    fn test_create_matcher_from_config_file_name_exact_match() {
        let find_command = crate::commands::find::FindCommand {
            file_name: Some("matched.txt".to_string()),
            is_regex: false,
            ..Default::default()
        };
        let cli = Cli::default();

        let file_matcher = create_matcher_from_config(&find_command, &cli).unwrap();
        let matched_file = get_dir_entry_from_path("filtering/matched");
        let not_matched_file = get_dir_entry_from_path("filtering/not_matched");

        assert!(file_matcher.matches(&matched_file, &cli).unwrap());
        assert!(!file_matcher.matches(&not_matched_file, &cli).unwrap());
    }

    #[test]
    fn test_create_matcher_from_config_file_name_regex_match() {
        let find_command = crate::commands::find::FindCommand {
            file_name: Some(r"^matched\.txt$".to_string()),
            is_regex: true,
            ..Default::default()
        };
        let cli = Cli::default();

        let file_matcher = create_matcher_from_config(&find_command, &cli).unwrap();
        let matched_file = get_dir_entry_from_path("filtering/matched");
        let not_matched_file = get_dir_entry_from_path("filtering/not_matched");

        assert!(file_matcher.matches(&matched_file, &cli).unwrap());
        assert!(!file_matcher.matches(&not_matched_file, &cli).unwrap());
    }

    #[test]
    fn test_create_matcher_from_config_size_exact_match() {
        let find_command = crate::commands::find::FindCommand {
            size: Some("1024".to_string()),
            is_regex: false,
            ..Default::default()
        };
        let cli = Cli::default();

        let file_matcher = create_matcher_from_config(&find_command, &cli).unwrap();
        let matched_file = get_dir_entry_from_path("filtering/size_1024");
        let not_matched_file = get_dir_entry_from_path("filtering/size_512");

        assert!(file_matcher.matches(&matched_file, &cli).unwrap());
        assert!(!file_matcher.matches(&not_matched_file, &cli).unwrap());
    }

    #[test]
    fn test_create_matcher_from_config_size_regex_match() {
        let find_command = crate::commands::find::FindCommand {
            size: Some("10..".to_string()),
            is_regex: true,
            ..Default::default()
        };
        let cli = Cli::default();

        let file_matcher = create_matcher_from_config(&find_command, &cli).unwrap();
        let matched_file = get_dir_entry_from_path("filtering/size_1024");
        let not_matched_file = get_dir_entry_from_path("filtering/size_512");

        assert!(file_matcher.matches(&matched_file, &cli).unwrap());
        assert!(!file_matcher.matches(&not_matched_file, &cli).unwrap());
    }

    #[test]
    fn test_create_matcher_from_config_ext_exact_match() {
        let find_command = crate::commands::find::FindCommand {
            ext: Some("txt".to_string()),
            is_regex: false,
            ..Default::default()
        };
        let cli = Cli::default();

        let file_matcher = create_matcher_from_config(&find_command, &cli).unwrap();
        let matched_file = get_dir_entry_from_path("filtering/ext_txt");
        let not_matched_file = get_dir_entry_from_path("filtering/ext_rs");

        assert!(file_matcher.matches(&matched_file, &cli).unwrap());
        assert!(!file_matcher.matches(&not_matched_file, &cli).unwrap());
    }

    #[test]
    fn test_create_matcher_from_config_ext_regex_match() {
        let find_command = crate::commands::find::FindCommand {
            ext: Some(r"^txt$".to_string()),
            is_regex: true,
            ..Default::default()
        };
        let cli = Cli::default();

        let file_matcher = create_matcher_from_config(&find_command, &cli).unwrap();
        let matched_file = get_dir_entry_from_path("filtering/ext_txt");
        let not_matched_file = get_dir_entry_from_path("filtering/ext_rs");

        assert!(file_matcher.matches(&matched_file, &cli).unwrap());
        assert!(!file_matcher.matches(&not_matched_file, &cli).unwrap());
    }

    #[test]
    fn test_create_matcher_from_config_ext_empty_match() {
        let find_command = crate::commands::find::FindCommand {
            ext: Some("".to_string()),
            is_regex: false,
            ..Default::default()
        };
        let cli = Cli::default();

        let file_matcher = create_matcher_from_config(&find_command, &cli).unwrap();
        let matched_file = get_dir_entry_from_path("filtering/no_ext");
        let not_matched_file = get_dir_entry_from_path("filtering/ext_txt");

        assert!(file_matcher.matches(&matched_file, &cli).unwrap());
        assert!(!file_matcher.matches(&not_matched_file, &cli).unwrap());
    }

    #[test]
    fn test_create_matcher_from_config_datetime_exact_match() {
        use time::format_description;

        let mut cli = Cli::default();
        cli.datetime_format =
            format_description::parse("[day]-[month]-[year] [hour]:[minute]").unwrap();

        // Create a temporary file for testing datetime
        let temp_dir = std::env::temp_dir();
        let test_file_path = temp_dir.join("test_datetime_file.txt");
        std::fs::write(&test_file_path, "test content").unwrap();

        // Get the creation datetime of the file
        let metadata = std::fs::metadata(&test_file_path).unwrap();
        let created = metadata.created().unwrap();
        let offset_datetime = time::OffsetDateTime::from(created);
        let primitive_datetime =
            PrimitiveDateTime::new(offset_datetime.date(), offset_datetime.time());

        let datetime_str = primitive_datetime.format(&cli.datetime_format).unwrap();

        let find_command = crate::commands::find::FindCommand {
            datetime: Some(datetime_str.clone()),
            is_regex: false,
            ..Default::default()
        };

        let file_matcher = create_matcher_from_config(&find_command, &cli).unwrap();
        let test_entry = std::fs::read_dir(&temp_dir)
            .unwrap()
            .find(|entry| {
                entry
                    .as_ref()
                    .unwrap()
                    .file_name()
                    .to_string_lossy()
                    .contains("test_datetime_file")
            })
            .unwrap()
            .unwrap();

        assert!(file_matcher.matches(&test_entry, &cli).unwrap());

        // Cleanup
        std::fs::remove_file(&test_file_path).ok();
    }

    #[test]
    fn test_create_matcher_from_config_datetime_regex_match() {
        use time::format_description;

        let mut cli = Cli::default();
        cli.datetime_format =
            format_description::parse("[day]-[month]-[year] [hour]:[minute]").unwrap();

        // Create a temporary file for testing datetime
        let temp_dir = std::env::temp_dir();
        let test_file_path = temp_dir.join("test_datetime_regex_file.txt");
        std::fs::write(&test_file_path, "test content").unwrap();

        // Get the creation datetime of the file
        let metadata = std::fs::metadata(&test_file_path).unwrap();
        let created = metadata.created().unwrap();
        let offset_datetime = time::OffsetDateTime::from(created);
        let primitive_datetime =
            PrimitiveDateTime::new(offset_datetime.date(), offset_datetime.time());

        let datetime_str = primitive_datetime.format(&cli.datetime_format).unwrap();
        // Create a regex pattern that matches the datetime (e.g., matches the date part)
        let date_part = &datetime_str[0..10]; // Get "DD-MM-YYYY" part
        let regex_pattern = format!(r"^{}.*", regex::escape(date_part));

        let find_command = crate::commands::find::FindCommand {
            datetime: Some(regex_pattern),
            is_regex: true,
            ..Default::default()
        };

        let file_matcher = create_matcher_from_config(&find_command, &cli).unwrap();
        let test_entry = std::fs::read_dir(&temp_dir)
            .unwrap()
            .find(|entry| {
                entry
                    .as_ref()
                    .unwrap()
                    .file_name()
                    .to_string_lossy()
                    .contains("test_datetime_regex_file")
            })
            .unwrap()
            .unwrap();

        assert!(file_matcher.matches(&test_entry, &cli).unwrap());

        // Cleanup
        std::fs::remove_file(&test_file_path).ok();
    }

    #[test]
    fn test_create_matcher_from_config_modified_exact_match() {
        use time::format_description;
        let mut cli = Cli::default();
        cli.datetime_format =
            format_description::parse("[day]-[month]-[year] [hour]:[minute]").unwrap();

        // Create a temporary file for testing modified datetime
        let temp_dir = std::env::temp_dir();
        let test_file_path = temp_dir.join("test_modified_file.txt");
        std::fs::write(&test_file_path, "test content").unwrap();

        // Get the modification datetime of the file
        let metadata = std::fs::metadata(&test_file_path).unwrap();
        let modified = metadata.created().unwrap(); // Using created as a proxy since we just created the file
        let offset_datetime = time::OffsetDateTime::from(modified)
            .replace_second(0)
            .unwrap();
        let primitive_datetime =
            PrimitiveDateTime::new(offset_datetime.date(), offset_datetime.time());

        let datetime_str = primitive_datetime.format(&cli.datetime_format).unwrap();

        let find_command = crate::commands::find::FindCommand {
            modified: Some(datetime_str.clone()),
            is_regex: false,
            ..Default::default()
        };

        let file_matcher = create_matcher_from_config(&find_command, &cli).unwrap();
        let test_entry = std::fs::read_dir(&temp_dir)
            .unwrap()
            .find(|entry| {
                entry
                    .as_ref()
                    .unwrap()
                    .file_name()
                    .to_string_lossy()
                    .contains("test_modified_file")
            })
            .unwrap()
            .unwrap();

        assert!(file_matcher.matches(&test_entry, &cli).unwrap());

        // Cleanup
        std::fs::remove_file(&test_file_path).ok();
    }

    #[test]
    fn test_create_matcher_from_config_modified_regex_match() {
        use time::format_description;

        let mut cli = Cli::default();
        cli.datetime_format =
            format_description::parse("[day]-[month]-[year] [hour]:[minute]").unwrap();

        // Create a temporary file for testing modified datetime
        let temp_dir = std::env::temp_dir();
        let test_file_path = temp_dir.join("test_modified_regex_file.txt");
        std::fs::write(&test_file_path, "test content").unwrap();

        // Get the modification datetime of the file
        let metadata = std::fs::metadata(&test_file_path).unwrap();
        let modified = metadata.created().unwrap(); // Using created as a proxy
        let offset_datetime = time::OffsetDateTime::from(modified);
        let primitive_datetime =
            PrimitiveDateTime::new(offset_datetime.date(), offset_datetime.time());

        let datetime_str = primitive_datetime.format(&cli.datetime_format).unwrap();
        // Create a regex pattern that matches the datetime
        let date_part = &datetime_str[0..10]; // Get "DD-MM-YYYY" part
        let regex_pattern = format!(r"^{}.*", regex::escape(date_part));

        let find_command = crate::commands::find::FindCommand {
            modified: Some(regex_pattern),
            is_regex: true,
            ..Default::default()
        };

        let file_matcher = create_matcher_from_config(&find_command, &cli).unwrap();
        let test_entry = std::fs::read_dir(&temp_dir)
            .unwrap()
            .find(|entry| {
                entry
                    .as_ref()
                    .unwrap()
                    .file_name()
                    .to_string_lossy()
                    .contains("test_modified_regex_file")
            })
            .unwrap()
            .unwrap();

        assert!(file_matcher.matches(&test_entry, &cli).unwrap());

        // Cleanup
        std::fs::remove_file(&test_file_path).ok();
    }
}
