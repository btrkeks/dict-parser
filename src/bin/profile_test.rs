use std::env;
use dict_parser::yomichan::yomichan_dictionary::YomichanDictionary;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1).cloned().unwrap_or_else(|| "./jitendex-yomitan.zip".to_string());

    let mut dictionary = YomichanDictionary::from_path(path)?;
    
    for _ in dictionary.term_banks_entries() {
        // Just iterate through all entries without processing
    }

    Ok(())
}