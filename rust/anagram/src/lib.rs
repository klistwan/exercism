use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lowercased_word = word.to_lowercase();
    let sorted_word = sort_word(&lowercased_word);

    possible_anagrams
        .iter()
        .filter(|elem| {
            let lowercased_elem = elem.to_lowercase();
            lowercased_elem != lowercased_word && sort_word(&lowercased_elem) == sorted_word
        })
        .copied()
        .collect()
}

fn sort_word(word: &str) -> Vec<char> {
    let mut sorted_word: Vec<char> = word.chars().collect();
    sorted_word.sort_unstable();
    sorted_word
}
