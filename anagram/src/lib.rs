use std::collections::{HashMap, HashSet};

fn char_count(word: &str) -> HashMap<char, u16> {
    let mut map: HashMap<char, u16> = HashMap::new();

    for c in word.to_lowercase().chars() {
        *map.entry(c).or_insert(0) += 1;
    }

    map
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let map = char_count(word);

    possible_anagrams
        .iter()
        .filter(|target| {
            target.to_lowercase() != word.to_lowercase() && map == char_count(target)
        })
        .map(|t| *t)
        .collect()
}
