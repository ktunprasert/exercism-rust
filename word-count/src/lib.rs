use std::collections::HashMap;

const SINGLE_QUOTE: char = '\'';

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    words
        .to_ascii_lowercase()
        .split(|c| match c {
            '\'' => false,
            _ => c.is_ascii_whitespace() || c.is_ascii_punctuation(),
        })
        .filter_map(|mut s| {
            if s.starts_with(SINGLE_QUOTE) {
                s = s.get(1..).unwrap_or(s)
            }

            if s.ends_with(SINGLE_QUOTE) && !s.is_empty() {
                s = s.get(..s.len() - 1).unwrap_or(s)
            }

            if s.is_empty() { None } else { Some(s) }
        })
        .map(String::from)
        .fold(HashMap::new(), |mut map, str| {
            map.entry(str).and_modify(|n| *n += 1).or_insert(1u32);

            map
        })
}
