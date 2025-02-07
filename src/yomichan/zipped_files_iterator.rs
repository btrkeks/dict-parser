use std::io::{Read, Seek};
use zip::read::ZipFile;
use zip::result::{ZipError, ZipResult};
use zip::ZipArchive;

pub struct TermBankFilesIterator<'a, R: Read + Seek> {
    i: usize,
    total: usize,
    archive: &'a mut ZipArchive<R>,
}

impl<'a, R: Read + Seek> TermBankFilesIterator<'a, R> {
    pub fn new(archive: &'a mut ZipArchive<R>) -> Self {
        Self {
            i: 0,
            total: archive.len(),
            archive,
        }
    }
}

impl<'a, R: Read + Seek> Iterator for TermBankFilesIterator<'a, R> {
    type Item = ZipResult<String>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.i < self.total {
            let current_index = self.i;
            self.i += 1;

            match self.archive.by_index(current_index) {
                Ok(mut file) if is_term_bank_file(&file) => {
                    let mut content = String::with_capacity(file.size() as usize);
                    return Some(file.read_to_string(&mut content)
                        .map(|_| content)
                        .map_err(ZipError::from));
                }
                Ok(_) => continue,  // Not a term bank file
                Err(e) => return Some(Err(e)),
            }
        }
        None
    }
}

#[inline]
fn is_term_bank_file(file: &ZipFile) -> bool {
    let filename = file.name();
    filename.ends_with(".json") && filename.starts_with("term_bank_")
}