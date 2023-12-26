pub fn reply(message: &str) -> &str {
    let alphas = message.chars().filter(|c| c.is_alphabetic()).collect::<Vec<char>>();
    let is_question = message.trim().ends_with('?');
    let yelling = !alphas.is_empty() && alphas.iter().all(|c| c.is_uppercase());

    match message.trim() {
        _m if is_question && yelling => "Calm down, I know what I'm doing!",
        _m if is_question => "Sure.",
        _m if yelling => "Whoa, chill out!",
        "" => "Fine. Be that way!",
        _ => "Whatever.",
    }
}
