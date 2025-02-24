use anyhow::Result;
use std::fs::{File};
use std::io::Read;
use std::path::{Path};
use serde::Deserialize;
use zip::ZipArchive;
use crate::yomichan::term_bank_iterator::{YomichanTermBankEntryIterator};
use crate::yomichan::term_bank_parsing::{parse_term_bank_from_bytes, YomichanTermBankEntry};
use rayon::prelude::*;
use crate::yomichan::zipped_files_iterator::is_term_bank_file;
use bumpalo::Bump;


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
        match archive.by_name("index.json") {
            Ok(file) => {
                // Parse metadata from index.json
                let metadata = serde_json::from_reader(file)?;
                Ok(metadata)
            }
            Err(_) => {
                // No metadata found, return default
                Ok(YomichanDictionaryMetaData::default())
            }
        }
    }

    pub fn get_name(&self) -> &str {
        &self.metadata.title
    }

    pub fn term_banks_entries(&mut self) -> impl Iterator<Item=YomichanTermBankEntry> {
        YomichanTermBankEntryIterator::new(&mut self.archive)
    }

    pub fn parallel_term_banks_entries(&mut self) -> Vec<YomichanTermBankEntry> {
        todo!()
        // // Get all term bank indices
        // let term_bank_indices: Vec<_> = (0..self.archive.len())
        //     .filter(|&i| {
        //         if let Ok(file) = self.archive.by_index(i) {
        //             is_term_bank_file(&file)
        //         } else {
        //             false
        //         }
        //     })
        //     .collect();
        //
        // // Process files in parallel
        // term_bank_indices.par_iter()
        //     .flat_map(|&idx| {
        //         if let Ok(mut file) = self.archive.by_index(idx) {
        //             let mut buf = Vec::with_capacity(file.size() as usize);
        //             buf.resize(file.size() as usize, 0);
        //
        //             if file.read_exact(&mut buf).is_ok() {
        //                 return parse_term_bank_from_bytes(&buf).unwrap_or_default();
        //             }
        //         }
        //         Vec::new() // Return empty vec on error
        //     })
        //     .collect()
    }
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
        // This is what needs to be optimized
        let mut dictionary = YomichanDictionary::from_path("jitendex-yomitan.zip").unwrap();
        println!("Entry count: {}", dictionary.term_banks_entries().count());
    }
}

