pub fn reply(message: &str) -> &str {
    let alphas = message.chars().filter(|c| c.is_alphabetic()).collect::<Vec<char>>();
    let is_question = message.trim().ends_with('?');
    let yelling = !alphas.is_empty() && alphas.iter().all(|c| c.is_uppercase());
    let silence = message.trim().is_empty();

    match (is_question, yelling, silence) {
        (true, true, _) => "Calm down, I know what I'm doing!",
        (true, _, _) => "Sure.",
        (_, true, _) => "Whoa, chill out!",
        (_, _, true) => "Fine. Be that way!",
        _ => "Whatever.",
    }
}
