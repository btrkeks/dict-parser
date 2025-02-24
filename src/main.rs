pub mod yomichan;
use crate::yomichan::yomichan_dictionary::YomichanDictionary;

fn main() {
    let mut dictionary = YomichanDictionary::from_path("./jitendex-yomitan.zip").unwrap();
    println!("Entry count: {}", dictionary.term_banks_entries().count());
}