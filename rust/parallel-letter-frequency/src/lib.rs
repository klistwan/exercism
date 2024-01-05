use std::collections::HashMap;
use std::thread;

type CharacterToFrequency = HashMap<char, usize>;

pub fn frequency(input: &[&str], worker_count: usize) -> CharacterToFrequency {
    // Divide input into chunks.
    let chunk_size = input.len() / worker_count + 1;
    let chunks: Vec<Vec<String>> = input
        .chunks(chunk_size)
        .map(|chunk| chunk.iter().map(|s| s.to_string()).collect())
        .collect();

    // Spawn workers.
    let mut workers = Vec::with_capacity(worker_count);
    for chunk in chunks {
        workers.push(thread::spawn(move || count_characters(&chunk)));
    }

    // Aggregate the results.
    let mut result = HashMap::new();
    for worker in workers {
        let w_result = worker.join().unwrap();
        for (&char, &count) in w_result.iter() {
            *result.entry(char).or_insert(0) += count;
        }
    }
    result
}

pub fn count_characters(lines: &[String]) -> CharacterToFrequency {
    let mut result = HashMap::new();
    for line in lines {
        for c in line.to_lowercase().chars().filter(|c| c.is_alphabetic()) {
            *result.entry(c).or_insert(0) += 1;
        }
    }
    result
}
