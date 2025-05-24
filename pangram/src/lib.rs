/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    sentence
        .to_ascii_lowercase()
        .chars()
        .fold([false; 26], |mut acc, c| match c {
            'a'..='z' => {
                acc[(c as u8 - b'a') as usize] = true;
                acc
            }
            _ => acc,
        })
        .iter()
        .all(|&b| b)
}
