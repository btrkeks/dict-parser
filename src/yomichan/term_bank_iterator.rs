use crate::yomichan::zipped_files_iterator::TermBankFilesIterator;
use std::fs::File;
use std::vec;
use zip::result::ZipError;
use zip::ZipArchive;
use crate::yomichan::term_bank_parsing::{parse_term_bank_from_string, YomichanTermBankEntry};

pub struct YomichanTermBankEntryIterator<'a> {
    term_bank_files_iterator: TermBankFilesIterator<'a, File>,
    current_entries: vec::IntoIter<YomichanTermBankEntry>,
}

impl<'a> YomichanTermBankEntryIterator<'a> {
    pub fn new(archive: &'a mut ZipArchive<File>) -> Self {
        Self {
            term_bank_files_iterator: TermBankFilesIterator::new(archive),
            current_entries: Vec::new().into_iter(),
        }
    }

    /// Indicates whether an error occurred during iteration.
    pub fn error(&mut self) -> Option<ZipError> {
        self.term_bank_files_iterator.error()
    }
}

impl<'a> Iterator for YomichanTermBankEntryIterator<'a> {
    type Item = YomichanTermBankEntry;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(entry) = self.current_entries.next() {
                return Some(entry);
            }

            let json_string = match self.term_bank_files_iterator.next() {
                Some(s) => s,
                None => return None,
            };

            match parse_term_bank_from_string(&json_string) {
                Ok(entries) => self.current_entries = entries.into_iter(),
                Err(e) => {
                    // TODO: Log error.
                    return  None;
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

    #[test]
    fn test_correctly_parses_term_bank_from_string() {
        let file = File::open("jitendex-yomitan.zip").unwrap();
        let mut archive = ZipArchive::new(file).unwrap();
        let mut term_bank_files_iterator = TermBankFilesIterator::new(&mut archive);

        while let Some(json_string) = term_bank_files_iterator.next() {
            black_box(parse_term_bank_from_string(&json_string).unwrap());
        }
    }
}
