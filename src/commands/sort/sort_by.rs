use std::{collections::HashMap, fs::DirEntry};

use clap::ValueEnum;

use crate::commands::sort::{sort_by_date, sort_by_extension, sort_by_size};

#[derive(Clone, Debug, Default, ValueEnum, Copy)]
pub enum SortBy {
    Size,
    Ext,
    #[default]
    Date,
}

impl SortBy {
    pub fn sort(self, files: Vec<DirEntry>) -> anyhow::Result<HashMap<String, Vec<DirEntry>>> {
        match self {
            SortBy::Size => sort_by_size(files),
            SortBy::Ext => sort_by_extension(files),
            SortBy::Date => sort_by_date(files),
        }
    }
}
