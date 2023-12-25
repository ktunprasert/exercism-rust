pub fn raindrops(n: u32) -> String {
    let map = [(3, "Pling"), (5, "Plang"), (7, "Plong")];

    let mut output = String::new();
    for (factor, sound) in map.iter() {
        if n % factor == 0 {
            output.push_str(sound);
        }
    }

    if output.is_empty() {
        n.to_string()
    } else {
        output
    }
}
