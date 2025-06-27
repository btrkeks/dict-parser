use crate::yomichan::zipped_files_iterator::TermBankFilesIterator;
use std::fs::File;
use std::vec;
use zip::result::ZipError;
use zip::ZipArchive;
use crate::yomichan::term_bank_parsing::{parse_term_bank_from_bytes, YomichanTermBankEntry};

/// An iterator over term bank entries in a Yomichan dictionary.
///
/// This iterator handles the low-level details of iterating through term bank
/// files in a Yomichan dictionary and parsing the entries from those files.
pub struct YomichanTermBankEntryIterator<'a> {
    term_bank_files_iterator: TermBankFilesIterator<'a, File>,
    current_entries: vec::IntoIter<YomichanTermBankEntry>,
    parse_error: Option<anyhow::Error>,
}

impl<'a> YomichanTermBankEntryIterator<'a> {
    /// Creates a new iterator from a zip archive.
    pub fn new(archive: &'a mut ZipArchive<File>) -> Self {
        Self {
            term_bank_files_iterator: TermBankFilesIterator::new(archive),
            current_entries: Vec::new().into_iter(),
            parse_error: None,
        }
    }

    /// Indicates whether a ZIP error occurred during iteration.
    pub fn error(&mut self) -> Option<ZipError> {
        self.term_bank_files_iterator.error()
    }

    /// Indicates whether a parsing error occurred during iteration.
    pub fn parse_error(&mut self) -> Option<anyhow::Error> {
        self.parse_error.take()
    }
}

impl<'a> Iterator for YomichanTermBankEntryIterator<'a> {
    type Item = YomichanTermBankEntry;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(entry) = self.current_entries.next() {
                return Some(entry);
            }

            let json_bytes = match self.term_bank_files_iterator.next() {
                Some(bytes) => bytes,
                None => return None,
            };

            // Parse directly from bytes
            match parse_term_bank_from_bytes(&json_bytes) {
                Ok(entries) => {
                    // Preallocate capacity for better performance
                    let mut vec = Vec::with_capacity(entries.len());
                    vec.extend(entries);
                    self.current_entries = vec.into_iter();
                }
                Err(e) => {
                    // Store error instead of panicking
                    self.parse_error = Some(e);
                    continue; // Continue to next file on error
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::hint::black_box;
    use anyhow::Result;

    #[test]
    fn test_correctly_parses_term_bank_from_string() -> Result<()> {
        let file = File::open("jitendex-yomitan.zip")?;
        let mut archive = ZipArchive::new(file)?;
        let mut term_bank_files_iterator = TermBankFilesIterator::new(&mut archive);

        while let Some(json_string) = term_bank_files_iterator.next() {
            black_box(parse_term_bank_from_bytes(&json_string)?);
        }

        Ok(())
    }

    #[test]
    fn test_iterator_error_handling() -> Result<()> {
        let file = File::open("jitendex-yomitan.zip")?;
        let mut archive = ZipArchive::new(file)?;
        let mut iterator = YomichanTermBankEntryIterator::new(&mut archive);

        // Consume some entries
        let mut count = 0;
        for _ in iterator.by_ref().take(100) {
            count += 1;
        }

        // Check for errors
        assert!(iterator.error().is_none(), "Unexpected ZIP error during iteration");
        assert!(iterator.parse_error().is_none(), "Unexpected parse error during iteration");
        assert!(count > 0, "Expected to find at least one entry");

        Ok(())
    }
}