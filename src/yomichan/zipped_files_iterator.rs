use std::io::{Read, Seek};
use zip::ZipArchive;
use zip::read::ZipFile;
use zip::result::ZipError;

pub struct TermBankFilesIterator<'a, R: Read + Seek> {
    term_bank_indices: Box<[usize]>,
    i: usize,
    archive: &'a mut ZipArchive<R>,
    error: Option<ZipError>,
    // Buffer used to store the contents of the current file.
    buf: String,
}

impl<'a, R: Read + Seek> TermBankFilesIterator<'a, R> {
    pub fn new(archive: &'a mut ZipArchive<R>) -> Self {
        Self {
            term_bank_indices: Self::term_bank_indices(archive),
            i: 0,
            archive,
            error: None,
            buf: String::new(),
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
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.term_bank_indices.len() {
            return None;
        }

        let archive_index = self.term_bank_indices[self.i];
        self.i += 1;

        match self.archive.by_index(archive_index) {
            Ok(mut file) => {
                self.buf.clear();

                if let Err(e) = file.read_to_string(&mut self.buf) {
                    self.error = Some(e.into());
                    return None;
                }

                Some(std::mem::take(&mut self.buf))
            }
            Err(e) => {
                self.error = Some(e.into());
                None
            }
        }
    }
}

#[inline]
fn is_term_bank_file(file: &ZipFile) -> bool {
    let filename = file.name();
    filename.ends_with(".json") && filename.starts_with("term_bank_")
}
