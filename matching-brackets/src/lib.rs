pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();

    for c in string.chars() {
        match c {
            '[' | '{' | '(' => stack.push(c),
            ']' | '}' | ')' => {
                if stack.is_empty() {
                    return false;
                }
                let last = stack.pop().unwrap();
                if !is_pair(last, c) {
                    return false;
                }
            }
            _ => continue,
        }
    }

    stack.is_empty()
}

pub fn is_pair(prev: char, current: char) -> bool {
    match (prev, current) {
        ('[', ']') => true,
        ('{', '}') => true,
        ('(', ')') => true,
        _ => false,
    }
}
