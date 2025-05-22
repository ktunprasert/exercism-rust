pub fn abbreviate(phrase: &str) -> String {
    let mut result = String::new();
    let mut prev: Option<char> = None;

    for c in phrase.chars() {
        if c == '_' {
            continue;
        }

        if prev.is_none() && c.is_alphabetic() {
            result.push(c.to_ascii_uppercase());
        } else if let Some(p) = prev {
            if (p == '-' || p.is_whitespace()) && c.is_alphanumeric() {
                result.push(c.to_ascii_uppercase());
            } else if p.is_ascii_lowercase() && c.is_ascii_uppercase() {
                result.push(c.to_ascii_uppercase());
            }
        }
        prev = Some(c);
    }

    result
}
