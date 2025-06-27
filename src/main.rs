//! A simple command-line tool for parsing dictionary files.

use std::env;
use std::time::Instant;
use dict_parser::yomichan::yomichan_dictionary::YomichanDictionary;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1).cloned().unwrap_or_else(|| "./jitendex-yomitan.zip".to_string());

    println!("Processing dictionary at {}", path);

    // Sequential processing
    let start = Instant::now();
    let mut dictionary = YomichanDictionary::from_path(path)?;
    let count = dictionary.term_banks_entries().count();
    let sequential_duration = start.elapsed();

    println!("Sequential processing: {} entries in {:?}", count, sequential_duration);

    // Parallel processing
    let start = Instant::now();
    let entries = dictionary.parallel_term_banks_entries()?;
    let parallel_duration = start.elapsed();

    println!("Parallel processing: {} entries in {:?}", entries.len(), parallel_duration);
    println!("Speedup: {:.2}x", sequential_duration.as_secs_f64() / parallel_duration.as_secs_f64());

    // Show some sample entries
    println!("\nSample entries:");
    for entry in entries.iter().take(5) {
        println!("Term: {}", entry.term);
        println!("Reading: {}", entry.reading);
        println!("Definitions:");
        for (i, def) in entry.definitions.iter().enumerate() {
            println!("  {}. {}", i+1, def);
        }
        println!();
    }

    Ok(())
}