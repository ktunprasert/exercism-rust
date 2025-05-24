pub fn rotate(input: &str, key: u8) -> String {
    if key == 0 || key == 26 {
        return String::from(input);
    }

    const LOW_RANGE: std::ops::RangeInclusive<char> = 'a'..='z';
    const UP_RANGE: std::ops::RangeInclusive<char> = 'A'..='Z';

    input
        .chars()
        .map(|c| {
            match c {
                'a'..='z' => Some((LOW_RANGE, (c as u8).saturating_sub(b'a') as usize)),
                'A'..='Z' => Some((UP_RANGE, (c as u8).saturating_sub(b'A') as usize)),
                _ => None,
            }
            .map(|(range, dist)| range.cycle().skip(dist).skip(key as usize).next().unwrap())
            .unwrap_or(c)
        })
        .collect()
}
