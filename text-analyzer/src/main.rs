use std::{collections::HashMap, env, error::Error, fs};
fn main() -> Result<(), Box<dyn Error>> {
    let file = read_file()?;

    let line_count = get_line_count(&file);
    println!("number of lines in file: {}", line_count);

    let word_count = get_word_count(&file);
    println!("number of words in file: {}", word_count);

    let freqs = get_frequency_unique_word(&file);
    
    println!("Unique word frequency list:");
    for (word, count) in freqs.iter() {
        println!("{}: {}", word, count);
    }
    Ok(())
}

fn read_file() -> Result<String, Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err("Usage: <program_name> <file_path>".into());
    }
    let file_path = &args[1];
    let file = fs::read_to_string(file_path)?;
    Ok(file)
}

fn get_line_count(file: &String) -> i64 {
    file.lines().count().try_into().unwrap()
}

fn get_word_count(file: &String) -> i64 {
    file.split_whitespace().count().try_into().unwrap()
}

fn get_frequency_unique_word(file: &String) -> HashMap<String, usize> {
    let mut word_counts = HashMap::new();

    for word in file.split_whitespace() {
        *word_counts.entry(word.to_string()).or_insert(0) += 1;
    }

    word_counts
}
