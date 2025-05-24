const PANGRAM: u32 = (1 << 26) - 1;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    sentence
        .chars()
        .try_fold(0, |mut seen, c| match c {
            _ if seen == PANGRAM => Err(()),
            'A'..='Z' | 'a'..='z' => {
                // 32 put both to be lowercase based
                let idx = (c as u8 | 32) - b'a';
                seen |= 1 << idx;
                Ok(seen)
            }
            _ => Ok(seen),
        })
        .map_or(true, |seen| seen == PANGRAM)
}
