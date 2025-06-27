use std::io::{Read, Seek};
use zip::ZipArchive;
use zip::read::ZipFile;
use zip::result::ZipError;

/// An iterator over term bank files in a Yomichan dictionary zip archive.
///
/// This iterator handles the low-level details of iterating through zip file entries
/// and filtering for term bank files. It yields the raw content of each term bank file.
pub struct TermBankFilesIterator<'a, R: Read + Seek> {
    term_bank_indices: Box<[usize]>,
    i: usize,
    archive: &'a mut ZipArchive<R>,
    error: Option<ZipError>,
    buf: Vec<u8>,
}

impl<'a, R: Read + Seek> TermBankFilesIterator<'a, R> {
    /// Creates a new iterator from a zip archive.
    pub fn new(archive: &'a mut ZipArchive<R>) -> Self {
        // Preallocate a reasonably sized buffer (1MB)
        let buf = Vec::with_capacity(1024 * 1024);

        Self {
            term_bank_indices: Self::term_bank_indices(archive),
            i: 0,
            archive,
            error: None,
            buf,
        }
    }

    /// Finds all term bank file indices in the zip archive.
    fn term_bank_indices(archive: &mut ZipArchive<R>) -> Box<[usize]> {
        (0..archive.len())
            .filter(|&i| {
                if let Ok(file) = archive.by_index(i) {
                    is_term_bank_file(&file)
                } else {
                    false
                }
            })
            .collect()
    }

    /// Indicates whether an error occurred during iteration.
    ///
    /// This method returns and consumes the first error that occurred during iteration.
    /// After calling this method, the error will be cleared from the iterator.
    pub fn error(&mut self) -> Option<ZipError> {
        self.error.take()
    }
}

impl<'a, R: Read + Seek> Iterator for TermBankFilesIterator<'a, R> {
    // Return Vec<u8> instead of String to avoid UTF-8 validation overhead
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.term_bank_indices.len() {
            return None;
        }

        let archive_index = self.term_bank_indices[self.i];
        self.i += 1;

        match self.archive.by_index(archive_index) {
            Ok(mut file) => {
                let size = file.size() as usize;

                // Clear and resize buffer - reuse allocation
                self.buf.clear();
                if self.buf.capacity() < size {
                    self.buf.reserve(size - self.buf.capacity());
                }
                self.buf.resize(size, 0);

                match file.read_exact(&mut self.buf) {
                    Ok(_) => {
                        // Create a new Vec from our buffer - unavoidable but cheaper than String conversion
                        Some(self.buf.clone())
                    }
                    Err(e) => {
                        // Store error instead of panicking
                        self.error = Some(e.into());
                        None
                    }
                }
            }
            Err(e) => {
                // Store error instead of panicking
                self.error = Some(e);
                None
            }
        }
    }
}

/// Determines whether a zip file entry is a term bank file.
#[inline]
pub fn is_term_bank_file(file: &ZipFile) -> bool {
    let filename = file.name();
    filename.ends_with(".json") && filename.starts_with("term_bank_")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use anyhow::Result;

    #[test]
    fn test_is_term_bank_file() {
        // Create a mock ZipFile with a given name
        struct MockZipFile {
            name: String,
        }

        impl MockZipFile {
            fn new(name: &str) -> Self {
                Self { name: name.to_string() }
            }

            fn name(&self) -> &str {
                &self.name
            }
        }

        // Test with valid term bank file names
        assert!(is_term_bank_file(&MockZipFile::new("term_bank_1.json")));
        assert!(is_term_bank_file(&MockZipFile::new("term_bank_123.json")));

        // Test with invalid term bank file names
        assert!(!is_term_bank_file(&MockZipFile::new("term_bank_1.txt")));
        assert!(!is_term_bank_file(&MockZipFile::new("bank_1.json")));
        assert!(!is_term_bank_file(&MockZipFile::new("index.json")));
    }

    #[test]
    fn test_iterator_yields_valid_data() -> Result<()> {
        let file = File::open("jitendex-yomitan.zip")?;
        let mut archive = ZipArchive::new(file)?;
        let mut iterator = TermBankFilesIterator::new(&mut archive);

        // Ensure we get at least one file
        let first_file = iterator.next();
        assert!(first_file.is_some(), "Expected at least one term bank file");

        // Ensure the data is not empty
        assert!(!first_file.unwrap().is_empty(), "Expected non-empty file");

        // Check for errors
        assert!(iterator.error().is_none(), "Unexpected error during iteration");

        Ok(())
    }
}