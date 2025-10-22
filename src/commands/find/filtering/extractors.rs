use std::fs::DirEntry;

use anyhow::bail;
use time::{OffsetDateTime, PrimitiveDateTime};

use crate::cli::Cli;

/// Trait for extracting values from DirEntry
pub trait Extractor<FilterValue> {
    fn extract(&self, file: &DirEntry, context: &Cli) -> anyhow::Result<FilterValue>;
}

/// File name extractor
pub struct FileNameExtractor;

impl Extractor<String> for FileNameExtractor {
    fn extract(&self, file: &DirEntry, _context: &Cli) -> anyhow::Result<String> {
        Ok(file.file_name().to_string_lossy().into_owned())
    }
}

/// File size extractor
pub struct FileSizeExtractor;

impl Extractor<u64> for FileSizeExtractor {
    fn extract(&self, file: &DirEntry, _context: &Cli) -> anyhow::Result<u64> {
        match file.metadata() {
            Ok(metadata) => Ok(metadata.len()),
            Err(err) => bail!("Failed to get file size: {}", err),
        }
    }
}

/// File size regex extractor
pub struct FileSizeRegexExtractor;

impl Extractor<String> for FileSizeRegexExtractor {
    fn extract(&self, file: &DirEntry, _context: &Cli) -> anyhow::Result<String> {
        match file.metadata() {
            Ok(metadata) => Ok(metadata.len().to_string()),
            Err(err) => bail!("Failed to get file size: {}", err),
        }
    }
}

/// File extension extractor
pub struct FileExtensionExtractor;

impl Extractor<String> for FileExtensionExtractor {
    fn extract(&self, file: &DirEntry, _context: &Cli) -> anyhow::Result<String> {
        match file.path().extension() {
            Some(ext) => Ok(ext.to_string_lossy().into_owned()),
            None => Ok("".to_string()),
        }
    }
}

/// File datetime extractor
pub struct FileDateExtractor;

impl Extractor<PrimitiveDateTime> for FileDateExtractor {
    fn extract(&self, file: &DirEntry, _context: &Cli) -> anyhow::Result<PrimitiveDateTime> {
        match file.metadata() {
            Ok(metadata) => match metadata.created() {
                Ok(system_time) => {
                    let offset_datetime = OffsetDateTime::from(system_time)
                        .replace_second(0)?
                        .replace_millisecond(0)?;

                    Ok(PrimitiveDateTime::new(
                        offset_datetime.date(),
                        offset_datetime.time(),
                    ))
                }
                Err(err) => bail!("Failed to get file creation date: {}", err),
            },
            Err(err) => bail!("Failed to get file metadata: {}", err),
        }
    }
}

/// File datetime regex extractor
pub struct FileDateRegexExtractor;

impl Extractor<String> for FileDateRegexExtractor {
    fn extract(&self, file: &DirEntry, context: &Cli) -> anyhow::Result<String> {
        match file.metadata() {
            Ok(metadata) => match metadata.created() {
                Ok(system_time) => {
                    let offset_datetime = OffsetDateTime::from(system_time);
                    let primitive_datetime =
                        PrimitiveDateTime::new(offset_datetime.date(), offset_datetime.time());

                    let formatted = primitive_datetime.format(&context.datetime_format)?;

                    Ok(formatted)
                }
                Err(err) => bail!("Failed to get file creation date: {}", err),
            },
            Err(err) => bail!("Failed to get file metadata: {}", err),
        }
    }
}
