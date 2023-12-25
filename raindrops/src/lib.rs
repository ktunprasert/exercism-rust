pub fn raindrops(n: u32) -> String {
    let result = [(3, "Pling"), (5, "Plang"), (7, "Plong")].iter().fold(
        String::with_capacity(15),
        |mut acc, (x, sound)| {
            if n % x == 0 {
                acc.push_str(sound)
            }

            acc
        },
    );

    if result.is_empty() {
        n.to_string()
    } else {
        result
    }
}
