const PANGRAM: u32 = (1 << 26) - 1;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    sentence
        .chars()
        .try_fold(0, |mut seen, c| match c {
            _ if seen == PANGRAM => Err(()),
            'A'..='Z' => {
                seen |= 1 << (c as u8 - b'A');
                Ok(seen)
            }
            'a'..='z' => {
                seen |= 1 << (c as u8 - b'a');
                Ok(seen)
            }
            _ => Ok(seen),
        })
        .map_or(true, |seen| seen == PANGRAM)
}
