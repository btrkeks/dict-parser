use std::io::{Read, Seek};
use zip::ZipArchive;
use zip::read::ZipFile;
use zip::result::ZipError;

pub struct TermBankFilesIterator<'a, R: Read + Seek> {
    term_bank_indices: Box<[usize]>,
    i: usize,
    archive: &'a mut ZipArchive<R>,
    error: Option<ZipError>,
    buf: Vec<u8>,
}

impl<'a, R: Read + Seek> TermBankFilesIterator<'a, R> {
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
                        self.error = Some(e.into());
                        None
                    }
                }
            }
            Err(e) => {
                self.error = Some(e.into());
                None
            }
        }
    }
}

#[inline]
pub fn is_term_bank_file(file: &ZipFile) -> bool {
    let filename = file.name();
    filename.ends_with(".json") && filename.starts_with("term_bank_")
}
