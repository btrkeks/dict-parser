use anyhow::Result;
use std::fs::{File};
use std::path::{Path};
use serde::Deserialize;
use zip::ZipArchive;
use crate::yomichan::term_bank_iterator::{YomichanTermBankEntryIterator};
use crate::yomichan::term_bank_parsing::YomichanTermBankEntry;

#[derive(Default, Debug, Deserialize)]
pub struct YomichanDictionaryMetaData {
    title: String,
    revision: String,
    author: String,
    sequenced: bool,
    format: u32,
    url: String,
    attribution: String,
}

pub struct YomichanDictionary {
    archive: ZipArchive<File>,
    metadata: YomichanDictionaryMetaData,
}

impl YomichanDictionary {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        let file = File::open(path)?;
        let mut archive = ZipArchive::new(file)?;
        let metadata = Self::extract_metadata(&mut archive)?;

        Ok(Self {
            archive,
            metadata,
        })
    }

    fn extract_metadata(archive: &mut ZipArchive<File>) -> Result<YomichanDictionaryMetaData> {
        // let index_file = archive.by_name("index.json")?;
        // Ok(serde_json::from_reader(index_file)?)
        // TODO
        Ok(YomichanDictionaryMetaData::default())
    }

    pub fn get_name(&self) -> &str {
        &self.metadata.title
    }

    pub fn term_banks_entries(&mut self) -> impl Iterator<Item=YomichanTermBankEntry> {
        YomichanTermBankEntryIterator::new(&mut self.archive)
    }

    // pub fn tag_bank_entries(&mut self) -> impl Iterator<Item=YomichanTagBankEntry> {
    //     YomichanTagBankEntryIterator::new(&mut self.archive)
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correctly_parses_metadata() {
        let dictionary = YomichanDictionary::from_path("jitendex-yomitan.zip").unwrap();
        assert_eq!(dictionary.get_name(), "Jitendex.org [2025-01-27]");
    }

    #[test]
    fn test_correctly_parses_term_bank() {
        let mut dictionary = YomichanDictionary::from_path("jitendex-yomitan.zip").unwrap();
        println!("Entry count: {}", dictionary.term_banks_entries().count());
    }
}

