pub fn rotate(input: &str, key: u8) -> String {
    if key == 0 || key == 26 {
        return input.chars().collect();
    }

    input
        .chars()
        .map(|c| {
            let c8: u8 = c as u8;

            match c {
                'a'..='z' => {
                    let dist: usize = c8.saturating_sub(b'a') as usize;

                    Some((('a'..='z'), dist))
                }
                'A'..='Z' => {
                    let dist = c8.saturating_sub(b'A') as usize;

                    Some((('A'..='Z'), dist))
                }
                _ => None,
            }
            .map(|(range, dist)| {
                range
                    .cycle()
                    .skip(dist)
                    .skip(key as usize)
                    .next()
                    .inspect(|c| println!("{:?}", c))
                    .unwrap()
            })
            .unwrap_or(c)
        })
        .collect()
}
