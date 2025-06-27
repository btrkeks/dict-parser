use anyhow::{Context, Result};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use serde::Deserialize;
use zip::ZipArchive;
use rayon::prelude::*;
use crate::yomichan::term_bank_iterator::YomichanTermBankEntryIterator;
use crate::yomichan::term_bank_parsing::{parse_term_bank_from_bytes, YomichanTermBankEntry};
use crate::yomichan::zipped_files_iterator::is_term_bank_file;

/// Metadata for a Yomichan dictionary.
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

/// A Yomichan dictionary loaded from a zip file.
pub struct YomichanDictionary {
    archive: ZipArchive<File>,
    metadata: YomichanDictionaryMetaData,
}

impl YomichanDictionary {
    /// Creates a new YomichanDictionary from a file path.
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        let file = File::open(&path)
            .with_context(|| format!("Failed to open dictionary file at {:?}", path.as_ref()))?;

        let mut archive = ZipArchive::new(file)
            .with_context(|| format!("Failed to parse zip archive at {:?}", path.as_ref()))?;

        let metadata = Self::extract_metadata(&mut archive)
            .with_context(|| "Failed to extract dictionary metadata")?;

        Ok(Self {
            archive,
            metadata,
        })
    }

    /// Extracts metadata from the dictionary zip file.
    fn extract_metadata(archive: &mut ZipArchive<File>) -> Result<YomichanDictionaryMetaData> {
        match archive.by_name("index.json") {
            Ok(file) => {
                // Parse metadata from index.json
                let metadata = serde_json::from_reader(file)
                    .with_context(|| "Failed to parse index.json")?;
                Ok(metadata)
            }
            Err(_) => {
                // No metadata found, return default
                Ok(YomichanDictionaryMetaData::default())
            }
        }
    }

    /// Returns the name of the dictionary.
    pub fn get_name(&self) -> &str {
        &self.metadata.title
    }

    /// Returns an iterator over entries in the term banks.
    pub fn term_banks_entries(&mut self) -> impl Iterator<Item=YomichanTermBankEntry> {
        YomichanTermBankEntryIterator::new(&mut self.archive)
    }

    /// Returns all entries in the term banks, processed in parallel.
    pub fn parallel_term_banks_entries(&mut self) -> Result<Vec<YomichanTermBankEntry>> {
        // Get all term bank indices
        let term_bank_indices: Vec<_> = (0..self.archive.len())
            .filter(|&i| {
                if let Ok(file) = self.archive.by_index(i) {
                    is_term_bank_file(&file)
                } else {
                    false
                }
            })
            .collect();

        // Read all term bank files into memory
        let term_bank_contents: Vec<Vec<u8>> = term_bank_indices
            .iter()
            .filter_map(|&idx| {
                if let Ok(mut file) = self.archive.by_index(idx) {
                    let mut buf = Vec::with_capacity(file.size() as usize);
                    buf.resize(file.size() as usize, 0);

                    if file.read_exact(&mut buf).is_ok() {
                        Some(buf)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        // Process files in parallel
        let results: Result<Vec<_>> = term_bank_contents
            .par_iter()
            .map(|bytes| {
                parse_term_bank_from_bytes(bytes)
                    .with_context(|| "Failed to parse term bank")
            })
            .collect();

        // Combine all entries into a single vector
        let entries = results?
            .into_iter()
            .flatten()
            .collect();

        Ok(entries)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_correctly_parses_metadata() -> Result<()> {
        let dictionary = YomichanDictionary::from_path("jitendex-yomitan.zip")?;
        assert_eq!(dictionary.get_name(), "Jitendex.org [2025-01-27]");
        Ok(())
    }

    #[test]
    fn test_correctly_parses_term_bank() -> Result<()> {
        let mut dictionary = YomichanDictionary::from_path("jitendex-yomitan.zip")?;
        let count = dictionary.term_banks_entries().count();
        println!("Entry count: {}", count);
        assert!(count > 0, "Expected at least one entry");
        Ok(())
    }

    #[test]
    fn test_parallel_term_banks_entries() -> Result<()> {
        let mut dictionary = YomichanDictionary::from_path("jitendex-yomitan.zip")?;

        // Get entries using both sequential and parallel methods
        let sequential_count = dictionary.term_banks_entries().count();
        let parallel_entries = dictionary.parallel_term_banks_entries()?;

        // Verify the counts match
        assert_eq!(sequential_count, parallel_entries.len(),
                   "Parallel processing should yield the same number of entries as sequential");

        // Verify we have entries
        assert!(parallel_entries.len() > 0, "Expected at least one entry");

        Ok(())
    }
}