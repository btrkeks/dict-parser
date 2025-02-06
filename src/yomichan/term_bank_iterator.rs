use crate::yomichan::zipped_files_iterator::TermBankFilesIterator;
use std::fs::File;
use zip::ZipArchive;
use crate::yomichan::term_bank_parsing::{parse_term_bank_from_string, YomichanTermBankEntry, YomichanTermBankEntryArray};

pub struct YomichanTermBankEntryIterator<'a> {
    term_bank_files_iterator: TermBankFilesIterator<'a, File>,
    current_entries: std::vec::IntoIter<YomichanTermBankEntry>,
}

impl<'a> YomichanTermBankEntryIterator<'a> {
    pub fn new(archive: &'a mut ZipArchive<File>) -> Self {
        Self {
            term_bank_files_iterator: TermBankFilesIterator::new(archive),
            current_entries: parse_term_bank_from_string("[]").unwrap().into_iter(), // empty term bank
        }
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
                Some(Ok(s)) => s,
                Some(Err(_)) => return None,
                None => return None,
            };

            match parse_term_bank_from_string(&json_string) {
                Ok(entries) => self.current_entries = entries.into_iter(),
                Err(e) => {
                    panic!("Failed to parse term bank: {}", e);
                    return None;
                }
            }
        }
    }
}
